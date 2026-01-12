//! Place a limit order on a perpetual market.
//!
//! This example demonstrates how to place a buy limit order on the BTC perpetual market.
//! It shows proper price handling, order configuration, and response parsing.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example send_order -- --private-key YOUR_PRIVATE_KEY
//! ```
//!
//! # What it does
//!
//! 1. Connects to Hyperliquid mainnet
//! 2. Finds the BTC perpetual market
//! 3. Places a buy limit order at $87,000 for 0.01 BTC
//! 4. Uses ALO (Add Liquidity Only) to ensure maker execution
//! 5. Prints the order response with order ID
//!
//! # Order Configuration
//!
//! - Market: BTC perpetual
//! - Side: Buy
//! - Price: $87,000
//! - Size: 0.01 BTC
//! - Type: Limit with ALO (Add Liquidity Only)
//! - Reduce Only: false (can increase position)

use std::env::home_dir;

use clap::Parser;
use hypersdk::hypercore::{
    self as hypercore, BatchCancel, BatchModify, Cancel, Cloid, Modify, NonceHandler, OidOrCloid,
    types::{BatchOrder, OrderGrouping, OrderRequest, OrderTypePlacement, TimeInForce},
};
use rust_decimal::dec;

use crate::credentials::Credentials;

mod credentials;

#[derive(Parser, Debug, derive_more::Deref)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[deref]
    #[command(flatten)]
    common: Credentials,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = simple_logger::init_with_level(log::Level::Debug);

    let args = Cli::parse();
    let signer = args.get()?;

    let client = hypercore::mainnet();
    let perps = client.perps().await?;
    let btc = perps.iter().find(|perp| perp.name == "BTC").expect("btc");

    let nonce = NonceHandler::default();

    let resp = client
        .place(
            &signer,
            BatchOrder {
                orders: vec![OrderRequest {
                    asset: btc.index,
                    is_buy: true,
                    limit_px: dec!(87_000),
                    sz: dec!(0.01),
                    reduce_only: false,
                    order_type: OrderTypePlacement::Limit {
                        tif: TimeInForce::Alo,
                    },
                    cloid: Cloid::random(),
                }],
                grouping: OrderGrouping::Na,
            },
            nonce.next(),
            None,
            None,
        )
        .await?;

    match &resp[0] {
        hypercore::OrderResponseStatus::Resting { oid, cloid: _cloid } => {
            let resp = client
                .modify(
                    &signer,
                    BatchModify {
                        modifies: vec![Modify {
                            oid: OidOrCloid::Left(*oid),
                            order: OrderRequest {
                                asset: btc.index,
                                is_buy: true,
                                limit_px: dec!(88_000),
                                sz: dec!(0.01),
                                reduce_only: false,
                                order_type: OrderTypePlacement::Limit {
                                    tif: TimeInForce::Alo,
                                },
                                cloid: Cloid::random(),
                            },
                        }],
                    },
                    nonce.next(),
                    None,
                    None,
                )
                .await?;

            match &resp[0] {
                hypercore::OrderResponseStatus::Resting { oid, cloid: _cloid } => {
                    client
                        .cancel(
                            &signer,
                            BatchCancel {
                                cancels: vec![Cancel {
                                    asset: btc.index,
                                    oid: *oid,
                                }],
                            },
                            nonce.next(),
                            None,
                            None,
                        )
                        .await?;
                }
                _ => {
                    println!("failed amending order: {resp:?}")
                }
            }
        }
        _ => {
            println!("failed placing order: {resp:?}")
        }
    }

    Ok(())
}
