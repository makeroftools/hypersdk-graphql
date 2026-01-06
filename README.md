# hypersdk

A comprehensive Rust SDK for interacting with the [Hyperliquid](https://hyperliquid.xyz) protocol.

[![Crates.io](https://img.shields.io/crates/v/hypersdk.svg)](https://crates.io/crates/hypersdk)
[![Documentation](https://docs.rs/hypersdk/badge.svg)](https://docs.rs/hypersdk)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)

## Overview

Hyperliquid is a high-performance decentralized exchange with two main components:

- **HyperCore**: The native L1 chain with perpetual and spot markets
- **HyperEVM**: An Ethereum-compatible layer for DeFi integrations (Morpho, Uniswap, etc.)

This SDK provides:

- Full HyperCore API support (HTTP and WebSocket)
- Trading operations (orders, cancellations, modifications)
- Real-time market data via WebSocket subscriptions
- Asset transfers between perps, spot, and EVM
- HyperEVM contract interactions (Morpho, Uniswap)
- Type-safe EIP-712 signing for all operations
- Accurate price tick rounding for orders

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
hypersdk = "0.1"
```

## Quick Start

### HyperCore - Query Markets

```rust
use hypersdk::hypercore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a mainnet client
    let client = hypercore::mainnet();

    // Get perpetual markets
    let perps = client.perps().await?;
    for market in perps {
        println!("{}: {}x leverage", market.name, market.max_leverage);
    }

    // Get spot markets
    let spots = client.spot().await?;
    for market in spots {
        println!("{}", market.symbol());
    }

    Ok(())
}
```

### HyperCore - Place an Order

```rust
use hypersdk::hypercore::{self, types::*, PrivateKeySigner};
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = hypercore::mainnet();
    let signer: PrivateKeySigner = "your_private_key".parse()?;

    let order = BatchOrder {
        orders: vec![OrderRequest {
            asset: 0, // BTC
            is_buy: true,
            limit_px: dec!(50000),
            sz: dec!(0.1),
            reduce_only: false,
            order_type: OrderTypePlacement::Limit {
                tif: TimeInForce::Gtc,
            },
            cloid: Default::default(),
        }],
        grouping: OrderGrouping::Na,
    };

    let nonce = chrono::Utc::now().timestamp_millis() as u64;
    let result = client.place(&signer, order, nonce, None, None).await?;

    println!("Order placed: {:?}", result);
    Ok(())
}
```

### HyperCore - WebSocket Subscriptions

```rust
use hypersdk::hypercore::{self, types::*};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut ws = hypercore::mainnet_ws();

    // Subscribe to market data
    ws.subscribe(Subscription::Trades { coin: "BTC".into() });
    ws.subscribe(Subscription::L2Book { coin: "ETH".into() });

    // Process incoming messages
    while let Some(msg) = ws.next().await {
        match msg {
            Incoming::Trades(trades) => {
                for trade in trades {
                    println!("{} @ {} size {}", trade.side, trade.px, trade.sz);
                }
            }
            Incoming::L2Book(book) => {
                println!("Order book update for {}", book.coin);
            }
            _ => {}
        }
    }

    Ok(())
}
```

### HyperEVM - Morpho Lending

```rust
use hypersdk::hyperevm::morpho;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = morpho::Client::mainnet().await?;

    // Get highest APY vault
    let vaults = client.highest_apy_vaults(10).await?;
    for vault in vaults {
        println!("{}: {:.2}% APY", vault.name, vault.apy * 100.0);
    }

    // Get specific market APY
    let apy = client.apy(morpho_address, market_id).await?;
    println!("Borrow APY: {:.2}%", apy.borrow * 100.0);
    println!("Supply APY: {:.2}%", apy.supply * 100.0);

    Ok(())
}
```

### HyperEVM - Uniswap V3

```rust
use hypersdk::hyperevm::uniswap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let contracts = uniswap::Contracts::mainnet();
    let client = uniswap::Client::mainnet(contracts).await?;

    // Get pool price
    let price = client.get_pool_price(token0, token1, 3000).await?;
    println!("Pool price: {}", price);

    // Get user positions
    let positions = client.positions(user_address).await?;
    for pos in positions {
        println!("Position #{}: {} liquidity", pos.token_id, pos.liquidity);
    }

    Ok(())
}
```

## Examples

The repository includes numerous examples demonstrating various features:

### HyperCore Examples

```bash
# List all perpetual and spot markets
cargo run --example list_markets

# List all spot tokens
cargo run --example list_tokens

# Place an order
cargo run --example send_order

# Transfer USDC
cargo run --example send_usd

# WebSocket market data
cargo run --example websocket

# Cross-chain transfers
cargo run --example transfer_to_evm
cargo run --example transfer_from_evm
cargo run --example transfer_to_perps
cargo run --example transfer_to_spot

# Combined workflow
cargo run --example buy_and_transfer
```

### Morpho Examples

```bash
# Get highest APY vaults
cargo run --example morpho_highest_apy

# Get supply/borrow APY for markets
cargo run --example morpho_supply_apy
cargo run --example morpho_borrow_apy

# Vault performance tracking
cargo run --example morpho_vault_apy
cargo run --example morpho_vault_performance

# Market creation events
cargo run --example morpho_create_market_events
```

### Uniswap Examples

```bash
# Query pool creation events
cargo run --example uniswap_pools_created

# Track token flows for PRJX
cargo run --example uniswap_prjx_flows
```

## Features

### Price Tick Rounding

The SDK includes accurate price tick size calculation for both spot and perpetual markets:

- **Perpetual markets**: 5 significant figures with max 6 decimal places
- **Spot markets**: 8 decimal places max with range-based tick sizes

```rust
use hypersdk::hypercore;

let client = hypercore::mainnet();
let perps = client.perps().await?;

// Get BTC market and round a price
let btc = perps.iter().find(|m| m.name == "BTC").unwrap();
// Price 93231.23 rounds to 93231 for BTC perps
```

### WebSocket Subscriptions

Subscribe to real-time market data:

```rust
use hypersdk::hypercore::types::Subscription;

// Available subscriptions:
Subscription::AllMids               // All mid prices
Subscription::Notification { user } // User notifications
Subscription::WebData { user }      // User web data
Subscription::Candle { coin, interval } // OHLCV candles
Subscription::L2Book { coin }       // Order book
Subscription::Trades { coin }       // Trade feed
Subscription::OrderUpdates { user } // Order updates
Subscription::UserEvents { user }   // User events
Subscription::UserFills { user }    // Fill events
Subscription::UserFundings { user } // Funding payments
Subscription::UserNonFundingLedgerUpdates { user } // Balance updates
```

### Cross-Chain Transfers

Transfer assets between HyperCore and HyperEVM:

```rust
use hypersdk::hypercore::{self, PrivateKeySigner};
use rust_decimal_macros::dec;

let client = hypercore::mainnet();
let signer: PrivateKeySigner = "your_private_key".parse()?;

// Transfer to EVM
client.transfer_to_evm(
    &signer,
    dec!(100.0),  // amount
    "USDC",       // token
    nonce
).await?;

// Transfer from EVM
client.transfer_from_evm(
    &signer,
    dec!(100.0),
    "USDC",
    nonce
).await?;
```

## Configuration

Most examples require a private key set via environment variable:

```bash
export PRIVATE_KEY="your_private_key_here"
```

For development, you can use a `.env` file:

```bash
PRIVATE_KEY=your_private_key_here
```

## Documentation

- [API Documentation](https://docs.rs/hypersdk)
- [Hyperliquid Documentation](https://hyperliquid.gitbook.io/hyperliquid-docs/)
- [Examples](./examples/)

## Requirements

- Rust 1.85.0 or higher
- Tokio async runtime

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Disclaimer

This software is provided "as is", without warranty of any kind. Use at your own risk. Trading cryptocurrencies involves substantial risk of loss.

## Support

- GitHub Issues: [Report bugs or request features](https://github.com/infinitefield/hypersdk/issues)
- Documentation: [docs.rs/hypersdk](https://docs.rs/hypersdk)

---

**Note**: This SDK is not officially affiliated with Hyperliquid. It is a community-maintained project.
