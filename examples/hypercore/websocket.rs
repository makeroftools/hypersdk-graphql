use anyhow::Context;
use futures::StreamExt;
use hypersdk::hypercore::{
    self as hypercore,
    types::{Incoming, Subscription},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = simple_logger::init_with_level(log::Level::Debug);

    let core = hypercore::mainnet();
    let spot = core.spot().await.context("spot")?;

    let khype = spot
        .iter()
        .find(|spot| spot.tokens[0].name == "KHYPE")
        .unwrap();

    let mut ws = core.websocket();
    ws.subscribe(Subscription::AllMids { dex: None });

    while let Some(item) = ws.next().await {
        if let Incoming::AllMids { dex: _, mids } = item {
            if let Some(price) = mids.get(&khype.name) {
                println!(
                    "Price of {}/{} is {}",
                    khype.tokens[0].name, khype.tokens[1].name, price
                );
            }
        }
    }

    Ok(())
}
