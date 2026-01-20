use hypersdk::hypercore;
use hypersdk::hypercore::{
    Error,
    BasicOrder
};
use hypersdk::{
    Address
};
// use tokio::io::stdout;

// async fn perp_dexs() -> anyhow::Result<()> {    
//     let client = hypercore::mainnet();
//     let dexes = client.perp_dexs().await?;
//     for dex in dexes {
//         println!("\n\nmarkets for {dex}");
//         let markets = client.perps_from(dex).await?;
//         for market in markets {
//             println!(
//                 "{}\t{}\t{}\t{}",
//                 market.name, market.index, market.name, market.collateral,
//             );
//         }
//     }
//     Ok(())
// }

async fn open_orders(address: Address) -> Result<Vec<BasicOrder>, anyhow::Error> {    
    let client = hypercore::mainnet();
    let orders = client.open_orders(address).await?;
    Ok(orders)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let user_address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
    let address = Address::parse_checksummed(user_address, None).expect("valid checksum");
    // let blah = perp_dexs().await?;
    let orders = open_orders(address).await?;
    println!("Num Orders: {:#?}", orders.len());
    for order in orders {
        println!("{:#?}\t{:#?}\t{:#?}\t{:#?}", order.coin, order.side, order.orig_sz, order.cloid);
    }
    
    Ok(())
}
