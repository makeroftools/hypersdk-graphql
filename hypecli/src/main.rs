use std::io::Write;
use std::io::stdout;

use clap::Args;
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use hypersdk::Address;
use hypersdk::hypercore;

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    // with_url: Url,
}

#[enum_dispatch]
trait Run {
    async fn run(&self) -> anyhow::Result<()>;
}

#[derive(Subcommand)]
#[enum_dispatch(Run)]
enum Commands {
    /// List perpetual markets
    Perps(PerpsCmd),
    /// List spot markets
    Spot(SpotCmd),
    /// Gather spot balances for a user.
    SpotBalances(SpotBalancesCmd),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    args.command.run().await
}

#[derive(Args)]
struct PerpsCmd;

impl Run for PerpsCmd {
    async fn run(&self) -> anyhow::Result<()> {
        let core = hypercore::mainnet();
        let perps = core.perps().await?;
        let mut writer = tabwriter::TabWriter::new(stdout());

        let _ = writeln!(
            &mut writer,
            "name\tcollateral\tindex\tsz_decimals\tmax leverage\tisolated margin"
        );
        for perp in perps {
            let _ = writeln!(
                &mut writer,
                "{}\t{}\t{}\t{}\t{}\t{}",
                perp.name,
                perp.collateral,
                perp.index,
                perp.sz_decimals,
                perp.max_leverage,
                perp.isolated_margin,
            );
        }

        let _ = writer.flush();

        Ok(())
    }
}

#[derive(Args)]
struct SpotCmd;

impl Run for SpotCmd {
    async fn run(&self) -> anyhow::Result<()> {
        let core = hypercore::mainnet();
        let markets = core.spot().await?;
        let mut writer = tabwriter::TabWriter::new(stdout());

        writeln!(
            &mut writer,
            "pair\tname\tindex\tbase evm address\tquote evm address"
        )?;
        for spot in markets {
            writeln!(
                &mut writer,
                "{}/{}\t{}\t{}\t{:?}\t{:?}",
                spot.tokens[0].name,
                spot.tokens[1].name,
                spot.name,
                spot.index,
                spot.tokens[0].evm_contract,
                spot.tokens[1].evm_contract,
            )?;
        }

        writer.flush()?;

        Ok(())
    }
}

#[derive(Args)]
struct SpotBalancesCmd {
    user: Address,
}

impl Run for SpotBalancesCmd {
    async fn run(&self) -> anyhow::Result<()> {
        let core = hypercore::mainnet();
        let balances = core.user_balances(self.user).await?;
        let mut writer = tabwriter::TabWriter::new(stdout());

        writeln!(&mut writer, "coin\thold\ttotal")?;
        for balance in balances {
            writeln!(
                &mut writer,
                "{}\t{}\t{}",
                balance.coin, balance.hold, balance.total
            )?;
        }

        writer.flush()?;

        Ok(())
    }
}
