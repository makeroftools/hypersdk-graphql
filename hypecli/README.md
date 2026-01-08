# hypecli

A command-line interface for interacting with the [Hyperliquid](https://app.hyperliquid.xyz) protocol.

[![Crates.io](https://img.shields.io/crates/v/hypecli.svg)](https://crates.io/crates/hypecli)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)

## Overview

`hypecli` is a lightweight CLI tool built on top of [hypersdk](https://github.com/infinitefield/hypersdk) for quick queries and operations on Hyperliquid. It provides fast access to market data, user balances, and DeFi protocol information without writing custom code.

## Installation

### From crates.io

```bash
cargo install hypecli
```

### From source

```bash
git clone https://github.com/infinitefield/hypersdk.git
cd hypersdk/hypecli
cargo install --path .
```

## Usage

```bash
hypecli --help
```

### Commands

#### List Perpetual Markets

Display all available perpetual futures markets with their details:

```bash
hypecli perps
```

**Output:**
```
name   collateral   index   sz_decimals   max leverage   isolated margin
BTC    0            0       5             50             100
ETH    0            1       4             50             100
SOL    0            2       1             20             50
...
```

**Columns explained:**
- `name`: Market symbol (e.g., BTC, ETH)
- `collateral`: Collateral token index
- `index`: Market index number
- `sz_decimals`: Number of decimals for size precision
- `max leverage`: Maximum allowed leverage
- `isolated margin`: Maximum isolated margin percentage

---

#### List Spot Markets

Display all available spot trading pairs:

```bash
hypecli spot
```

**Output:**
```
pair          name       index   base evm address                              quote evm address
PURR/USDC     PURR-SPOT  0       Some(0x4...)                                 Some(0x...)
HYPE/USDC     HYPE-SPOT  1       Some(0x2...)                                 Some(0x...)
...
```

**Columns explained:**
- `pair`: Trading pair (BASE/QUOTE)
- `name`: Spot market name
- `index`: Market index number
- `base evm address`: EVM contract address for base token (if available)
- `quote evm address`: EVM contract address for quote token (if available)

---

#### Query Spot Balances

Check spot token balances for a specific address:

```bash
hypecli spot-balances --user 0x1234567890abcdef1234567890abcdef12345678
```

**Output:**
```
coin    hold        total
USDC    0           1234.5678
HYPE    10.5        100.25
PURR    0           500.0
```

**Columns explained:**
- `coin`: Token symbol
- `hold`: Amount locked in orders or other operations
- `total`: Total balance (including held amount)

**Available balance** = `total - hold`

---

#### Query Morpho Position

Check a user's lending position on Morpho (HyperEVM):

```bash
hypecli morpho-position \
  --user 0x1234567890abcdef1234567890abcdef12345678 \
  --market 0xabcd...1234
```

**Output:**
```
borrow shares   collateral   supply shares
0               1000000000   5000000000
```

**Columns explained:**
- `borrow shares`: Amount of borrow shares held
- `collateral`: Collateral amount deposited
- `supply shares`: Amount of supply shares held

**Optional flags:**
- `--contract`: Morpho contract address (default: `0x68e37dE8d93d3496ae143F2E900490f6280C57cD`)
- `--rpc-url`: Custom RPC endpoint (default: `https://rpc.hyperliquid.xyz/evm`)

---

## Examples

### Market Research

```bash
# Quick check of available markets
hypecli perps | grep -E "BTC|ETH|SOL"

# Find spot markets with high liquidity
hypecli spot
```

### Portfolio Monitoring

```bash
# Check your balances
hypecli spot-balances --user YOUR_ADDRESS

# Monitor multiple addresses
for addr in $ADDRESS1 $ADDRESS2 $ADDRESS3; do
  echo "Balances for $addr:"
  hypecli spot-balances --user $addr
  echo ""
done
```

### DeFi Integration

```bash
# Check Morpho lending position
hypecli morpho-position \
  --user YOUR_ADDRESS \
  --market MARKET_ID
```

## Output Format

All commands output data in tab-separated format for easy parsing:

```bash
# Save to CSV
hypecli perps | tr '\t' ',' > perps.csv

# Filter with awk
hypecli spot | awk -F'\t' '$5 == "50" {print $1}'

# Parse with jq (convert to JSON first)
hypecli perps | awk 'NR==1 {for(i=1;i<=NF;i++) h[i]=$i; next} {printf "{"; for(i=1;i<=NF;i++) printf "\"%s\":\"%s\"%s", h[i], $i, (i<NF?",":""); print "}"}' | jq .
```

## Scripting Examples

### Bash: Find high-leverage markets

```bash
#!/bin/bash
hypecli perps | awk -F'\t' 'NR>1 && $5 >= 50 {print $1, ":", $5"x leverage"}'
```

### Python: Parse balances

```python
import subprocess
import csv

result = subprocess.run(['hypecli', 'spot-balances', '--user', address], 
                       capture_output=True, text=True)
reader = csv.DictReader(result.stdout.split('\n'), delimiter='\t')
for row in reader:
    print(f"{row['coin']}: {row['total']}")
```

### Rust: Use as library

```rust
// hypecli uses hypersdk internally, so you can use hypersdk directly
use hypersdk::hypercore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = hypercore::mainnet();
    let perps = client.perps().await?;
    println!("{} markets available", perps.len());
    Ok(())
}
```

## Network Selection

Currently, `hypecli` connects to **Hyperliquid mainnet** by default. Testnet support and custom RPC endpoints will be added in future releases.

For Morpho queries, you can specify a custom RPC:

```bash
hypecli morpho-position \
  --rpc-url https://your-custom-rpc.com \
  --user ADDRESS \
  --market MARKET_ID
```

## Future Features

The CLI will be extended with additional functionality:

- [ ] Trading operations (place orders, cancel orders)
- [ ] WebSocket subscriptions for real-time data
- [ ] Asset transfers (perps ↔ spot ↔ EVM)
- [ ] Multi-sig transaction building
- [ ] Testnet support via `--testnet` flag
- [ ] JSON output format via `--json` flag
- [ ] Historical data queries
- [ ] Portfolio analytics

## Development

### Building from source

```bash
cd hypecli
cargo build --release
./target/release/hypecli --help
```

### Adding new commands

1. Define a new struct implementing `Args`:
   ```rust
   #[derive(Args)]
   struct NewCmd {
       #[arg(short, long)]
       param: String,
   }
   ```

2. Implement the `Run` trait:
   ```rust
   impl Run for NewCmd {
       async fn run(&self) -> anyhow::Result<()> {
           // Command logic here
           Ok(())
       }
   }
   ```

3. Add to `Commands` enum:
   ```rust
   #[derive(Subcommand)]
   #[enum_dispatch(Run)]
   enum Commands {
       // ...
       NewCommand(NewCmd),
   }
   ```

## Dependencies

- [hypersdk](https://github.com/infinitefield/hypersdk) - Hyperliquid Rust SDK
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime
- [tabwriter](https://github.com/BurntSushi/tabwriter) - Aligned text output

## Documentation

- [hypersdk Documentation](https://docs.rs/hypersdk)
- [Hyperliquid API Docs](https://hyperliquid.gitbook.io/hyperliquid-docs/)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Ideas for contributions:
- New commands for trading operations
- JSON output format support
- Configuration file support
- Interactive mode
- Performance optimizations

## License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](../LICENSE) file for details.

## Support

- GitHub Issues: [Report bugs or request features](https://github.com/infinitefield/hypersdk/issues)
- Documentation: [docs.rs/hypersdk](https://docs.rs/hypersdk)

---

**Note**: This CLI is not officially affiliated with Hyperliquid. It is a community-maintained project built on hypersdk.
