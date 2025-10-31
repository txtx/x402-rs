//! Network definitions and known token deployments.
//!
//! This module defines supported networks and their chain IDs,
//! and provides statically known USDC deployments per network.

use crate::types::{MixedAddress, TokenAsset, TokenDeployment, TokenDeploymentEip712};
use alloy::primitives::address;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Network {
    Evm(EvmNetwork),
    Solana(SolanaNetwork),
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::Evm(network) => write!(f, "{network}"),
            Network::Solana(network) => write!(f, "{network}"),
        }
    }
}

impl NetworkTrait for Network {
    fn variants() -> &'static [Self] {
        static VARIANTS: Lazy<Vec<Network>> = Lazy::new(|| {
            let mut variants = Vec::new();
            for evm_network in EvmNetwork::variants() {
                variants.push(Network::Evm(*evm_network));
            }
            for solana_network in SolanaNetwork::variants() {
                variants.push(Network::Solana(*solana_network));
            }
            variants
        });
        &VARIANTS
    }
}

// impl NetworkTrait for Network removed because combining different `impl Trait`
// opaque types cannot be mixed; use `Network::variants()` for a combined list.

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvmNetwork {
    /// Base Sepolia testnet (chain ID 84532).
    #[serde(rename = "base-sepolia")]
    BaseSepolia,
    /// Base mainnet (chain ID 8453).
    #[serde(rename = "base")]
    Base,
    /// XDC mainnet (chain ID 50).
    #[serde(rename = "xdc")]
    XdcMainnet,
    /// Avalanche Fuji testnet (chain ID 43113)
    #[serde(rename = "avalanche-fuji")]
    AvalancheFuji,
    /// Avalanche Mainnet (chain ID 43114)
    #[serde(rename = "avalanche")]
    Avalanche,
    /// Polygon Amoy testnet (chain ID 80002).
    #[serde(rename = "polygon-amoy")]
    PolygonAmoy,
    /// Polygon mainnet (chain ID 137).
    #[serde(rename = "polygon")]
    Polygon,
    /// Sei mainnet (chain ID 1329).
    #[serde(rename = "sei")]
    Sei,
    /// Sei testnet (chain ID 1328).
    #[serde(rename = "sei-testnet")]
    SeiTestnet,
    #[serde(rename = "local")]
    Local,
}

impl NetworkTrait for EvmNetwork {
    fn variants() -> &'static [Self] {
        &[
            EvmNetwork::BaseSepolia,
            EvmNetwork::Base,
            EvmNetwork::XdcMainnet,
            EvmNetwork::AvalancheFuji,
            EvmNetwork::Avalanche,
            EvmNetwork::PolygonAmoy,
            EvmNetwork::Polygon,
            EvmNetwork::Sei,
            EvmNetwork::SeiTestnet,
        ]
    }
}

impl Display for EvmNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            EvmNetwork::BaseSepolia => "base-sepolia",
            EvmNetwork::Base => "base",
            EvmNetwork::XdcMainnet => "xdc",
            EvmNetwork::AvalancheFuji => "avalanche-fuji",
            EvmNetwork::Avalanche => "avalanche",
            EvmNetwork::PolygonAmoy => "polygon-amoy",
            EvmNetwork::Polygon => "polygon",
            EvmNetwork::Sei => "sei",
            EvmNetwork::SeiTestnet => "sei-testnet",
            EvmNetwork::Local => "local",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SolanaNetwork {
    /// Solana Mainnet - Live production environment for deployed applications
    #[serde(rename = "solana")]
    Mainnet,
    /// Solana Devnet - Testing with public accessibility for developers experimenting with their applications
    #[serde(rename = "solana-devnet")]
    Devnet,
    #[serde(rename = "cloud-surfnet")]
    CloudSurfnet,
    #[serde(rename = "local-surfnet")]
    LocalSurfnet,
}

impl NetworkTrait for SolanaNetwork {
    fn variants() -> &'static [Self] {
        &[
            SolanaNetwork::Mainnet,
            SolanaNetwork::Devnet,
            SolanaNetwork::CloudSurfnet,
            SolanaNetwork::LocalSurfnet,
        ]
    }
}

impl Display for SolanaNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            SolanaNetwork::Mainnet => "solana",
            SolanaNetwork::Devnet => "solana-devnet",
            SolanaNetwork::CloudSurfnet => "cloud-surfnet",
            SolanaNetwork::LocalSurfnet => "local-surfnet",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NetworkFamily {
    Evm,
    Solana,
}

impl From<Network> for NetworkFamily {
    fn from(value: Network) -> Self {
        match value {
            Network::Evm(network) => match network {
                EvmNetwork::BaseSepolia
                | EvmNetwork::Base
                | EvmNetwork::XdcMainnet
                | EvmNetwork::AvalancheFuji
                | EvmNetwork::Avalanche
                | EvmNetwork::PolygonAmoy
                | EvmNetwork::Polygon
                | EvmNetwork::Sei
                | EvmNetwork::SeiTestnet
                | EvmNetwork::Local => NetworkFamily::Evm,
            },
            Network::Solana(network) => match network {
                SolanaNetwork::Mainnet
                | SolanaNetwork::Devnet
                | SolanaNetwork::CloudSurfnet
                | SolanaNetwork::LocalSurfnet => NetworkFamily::Solana,
            },
        }
    }
}

pub trait NetworkTrait: Sized {
    fn variants() -> &'static [Self];
}

/// Lazily initialized known USDC deployment on Base Sepolia as [`USDCDeployment`].
static USDC_BASE_SEPOLIA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x036CbD53842c5426634e7929541eC2318f3dCF7e").into(),
            network: Network::Evm(EvmNetwork::BaseSepolia),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Base mainnet as [`USDCDeployment`].
static USDC_BASE: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913").into(),
            network: Network::Evm(EvmNetwork::Base),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on XDC mainnet as [`USDCDeployment`].
static USDC_XDC: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x2A8E898b6242355c290E1f4Fc966b8788729A4D4").into(),
            network: Network::Evm(EvmNetwork::XdcMainnet),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "Bridged USDC(XDC)".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Avalanche Fuji testnet as [`USDCDeployment`].
static USDC_AVALANCHE_FUJI: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x5425890298aed601595a70AB815c96711a31Bc65").into(),
            network: Network::Evm(EvmNetwork::AvalancheFuji),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Avalanche Fuji testnet as [`USDCDeployment`].
static USDC_AVALANCHE: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xB97EF9Ef8734C71904D8002F8b6Bc66Dd9c48a6E").into(),
            network: Network::Evm(EvmNetwork::Avalanche),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USD Coin".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Solana mainnet as [`USDCDeployment`].
static USDC_SOLANA: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            ),
            network: Network::Solana(SolanaNetwork::Mainnet),
        },
        decimals: 6,
        eip712: None,
    })
});

static USDC_SOLANA_LOCAL_SURFNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            ),
            network: Network::Solana(SolanaNetwork::LocalSurfnet),
        },
        decimals: 6,
        eip712: None,
    })
});

static USDC_SOLANA_CLOUD_SURFNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
            ),
            network: Network::Solana(SolanaNetwork::CloudSurfnet),
        },
        decimals: 6,
        eip712: None,
    })
});

/// Lazily initialized known USDC deployment on Solana mainnet as [`USDCDeployment`].
static USDC_SOLANA_DEVNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: MixedAddress::Solana(
                Pubkey::from_str("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU").unwrap(),
            ),
            network: Network::Solana(SolanaNetwork::Devnet),
        },
        decimals: 6,
        eip712: None,
    })
});

/// Lazily initialized known USDC deployment on Polygon Amoy testnet as [`USDCDeployment`].
static USDC_POLYGON_AMOY: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x41E94Eb019C0762f9Bfcf9Fb1E58725BfB0e7582").into(),
            network: Network::Evm(EvmNetwork::PolygonAmoy),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// Lazily initialized known USDC deployment on Polygon mainnet as [`USDCDeployment`].
static USDC_POLYGON: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359").into(),
            network: Network::Evm(EvmNetwork::Polygon),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_SEI: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0xe15fC38F6D8c56aF07bbCBe3BAf5708A2Bf42392").into(),
            network: Network::Evm(EvmNetwork::Sei),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

static USDC_SEI_TESTNET: Lazy<USDCDeployment> = Lazy::new(|| {
    USDCDeployment(TokenDeployment {
        asset: TokenAsset {
            address: address!("0x4fCF1784B31630811181f670Aea7A7bEF803eaED").into(),
            network: Network::Evm(EvmNetwork::SeiTestnet),
        },
        decimals: 6,
        eip712: Some(TokenDeploymentEip712 {
            name: "USDC".into(),
            version: "2".into(),
        }),
    })
});

/// A known USDC deployment as a wrapper around [`TokenDeployment`].
#[derive(Clone, Debug)]
pub struct USDCDeployment(pub TokenDeployment);

impl Deref for USDCDeployment {
    type Target = TokenDeployment;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&USDCDeployment> for TokenDeployment {
    fn from(deployment: &USDCDeployment) -> Self {
        deployment.0.clone()
    }
}

impl From<USDCDeployment> for Vec<TokenAsset> {
    fn from(deployment: USDCDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl From<&USDCDeployment> for Vec<TokenAsset> {
    fn from(deployment: &USDCDeployment) -> Self {
        vec![deployment.asset.clone()]
    }
}

impl USDCDeployment {
    /// Return the known USDC deployment for the given network.
    ///
    /// Panic if the network is unsupported (not expected in practice).
    pub fn by_network<N: Borrow<Network>>(network: N) -> &'static USDCDeployment {
        match network.borrow() {
            Network::Evm(evm_network) => match evm_network {
                EvmNetwork::BaseSepolia => &USDC_BASE_SEPOLIA,
                EvmNetwork::Base => &USDC_BASE,
                EvmNetwork::XdcMainnet => &USDC_XDC,
                EvmNetwork::AvalancheFuji => &USDC_AVALANCHE_FUJI,
                EvmNetwork::Avalanche => &USDC_AVALANCHE,
                EvmNetwork::PolygonAmoy => &USDC_POLYGON_AMOY,
                EvmNetwork::Polygon => &USDC_POLYGON,
                EvmNetwork::Sei => &USDC_SEI,
                EvmNetwork::SeiTestnet => &USDC_SEI_TESTNET,
                EvmNetwork::Local => panic!("No known USDC deployment on local EVM network"),
            },
            Network::Solana(solana_network) => match solana_network {
                SolanaNetwork::Mainnet => &USDC_SOLANA,
                SolanaNetwork::Devnet => &USDC_SOLANA_DEVNET,
                SolanaNetwork::CloudSurfnet => &USDC_SOLANA_CLOUD_SURFNET,
                SolanaNetwork::LocalSurfnet => &USDC_SOLANA_LOCAL_SURFNET,
            },
        }
    }
}
