use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use dotenvy::dotenv;
use opentelemetry::trace::Status;
use std::env;
use tower_http::trace::TraceLayer;
use tracing::instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use x402_axum::{IntoPriceTag, X402Middleware};
use x402_rs::network::{Network, SolanaNetwork, USDCDeployment};
use x402_rs::telemetry::Telemetry;
use x402_rs::types::MixedAddress;
use x402_rs::types::pubkey::Pubkey;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _telemetry = Telemetry::new()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
        .register();

    let facilitator_url =
        env::var("FACILITATOR_URL").unwrap_or_else(|_| "https://facilitator.x402.rs".to_string());

    let sol_facilitator_public_key = MixedAddress::Solana(Pubkey::from_str_const(
        &env::var("SOL_FACILITATOR_PUBLIC_KEY")
            .ok()
            .unwrap_or("EGBQqKn968sVv5cQh5Cr72pSTHfxsuzq7o7asqYB5uEV".to_string()),
    ));

    println!(
        "Using Solana facilitator public key: {}",
        sol_facilitator_public_key
    );

    let x402 = X402Middleware::try_from(facilitator_url)
        .unwrap()
        .with_base_url(url::Url::parse("https://localhost:3000/").unwrap());
    let usdc_local_surfnet =
        USDCDeployment::by_network(Network::Solana(SolanaNetwork::LocalSurfnet))
            .pay_to(sol_facilitator_public_key.clone());
    let usdc_solana = USDCDeployment::by_network(Network::Solana(SolanaNetwork::Mainnet))
        .pay_to(sol_facilitator_public_key);

    let app = Router::new()
        .route(
            "/protected-route",
            get(my_handler).layer(
                x402.with_description("Premium API")
                    .with_mime_type("application/json")
                    .with_price_tag(usdc_solana.amount(0.0025).unwrap())
                    .or_price_tag(usdc_local_surfnet.amount(0.0025).unwrap()),
            ),
        )
        .layer(
            // Usual HTTP tracing
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "http_request",
                        otel.kind = "server",
                        otel.name = %format!("{} {}", request.method(), request.uri()),
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     span: &tracing::Span| {
                        span.record("status", tracing::field::display(response.status()));
                        span.record("latency", tracing::field::display(latency.as_millis()));
                        span.record(
                            "http.status_code",
                            tracing::field::display(response.status().as_u16()),
                        );

                        // OpenTelemetry span status
                        if response.status().is_success()
                            || response.status() == StatusCode::PAYMENT_REQUIRED
                        {
                            span.set_status(Status::Ok);
                        } else {
                            span.set_status(Status::error(
                                response
                                    .status()
                                    .canonical_reason()
                                    .unwrap_or("unknown")
                                    .to_string(),
                            ));
                        }

                        tracing::info!(
                            "status={} elapsed={}ms",
                            response.status().as_u16(),
                            latency.as_millis()
                        );
                    },
                ),
        );

    tracing::info!("Using facilitator on {}", x402.facilitator_url());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Can not start server");
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[instrument(skip_all)]
async fn my_handler() -> impl IntoResponse {
    (StatusCode::OK, "This is a VIP content!")
}
