use alloy::sol;
use clap::Parser;
use hypersdk::{
    Address, U256,
    hyperevm::{self, DynProvider},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Address of the vault contract.
    #[arg(
        short,
        long,
        default_value = "0x207ccaE51Ad2E1C240C4Ab4c94b670D438d2201C"
    )]
    contract_address: Address,
    /// RPC url
    #[arg(short, long, default_value = "http://127.0.0.1:8545")]
    rpc_url: String,
}

sol! {
    struct MarketParams {
        address loanToken;
        address collateralToken;
        address oracle;
        address irm;
        uint256 lltv;
    }

    struct Market {
        uint128 totalSupplyAssets;
        uint128 totalSupplyShares;
        uint128 totalBorrowAssets;
        uint128 totalBorrowShares;
        uint128 lastUpdate;
        uint128 fee;
    }

    #[sol(rpc)]
    contract Morpho {
        function market(bytes32 market) returns (Market);
        function idToMarketParams(bytes32 market) returns (MarketParams);
    }

    #[sol(rpc)]
    contract MetaMorpho {
        bytes32[] public supplyQueue;

        function MORPHO() external view returns (address);
        function supplyQueueLength() external view returns (uint256);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    println!("Connecting to RPC endpoint: {}", args.rpc_url);

    let provider = DynProvider::new(hyperevm::mainnet_with_url(&args.rpc_url).await?);

    let meta_morpho = MetaMorpho::new(args.contract_address, provider.clone());
    let morpho = Morpho::new(meta_morpho.MORPHO().call().await?, provider.clone());
    let supply_queue_len = meta_morpho.supplyQueueLength().call().await?.to::<usize>();

    for i in 0..supply_queue_len {
        let market_id = meta_morpho.supplyQueue(U256::from(i)).call().await?;
        let params = morpho.idToMarketParams(market_id).call().await?;
        let market = morpho.market(market_id).call().await?;
        println!(
            "{}: {} / {} {} - {}",
            i,
            params.loanToken,
            params.collateralToken,
            market.totalBorrowAssets,
            market.totalSupplyAssets
        );
    }

    Ok(())
}
