mod web_utils;

use clap::Parser;
use log::{error, info};
use reqwest::Client;
use std::error;
use web_utils::construct_headers;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// The symbol you want to trade
    symbol: String,
    /// The quantity of the symbol you want to trade
    quantity: f32,
}

async fn market_buy_futures_position(
    client: Client,
    symbol: String,
    qty: f32,
    recv_window: &str,
) -> Result<(), Box<dyn error::Error>> {
    let url = "https://api-testnet.bybit.com/v5/order/create";

    let payload = format!(
        r#"{{"category":"linear","symbol":"{}","side":"Buy","orderType":"Market","qty":"{}"}}"#,
        symbol, qty
    );

    if let Ok(res) = client
        .post(url)
        .headers(construct_headers(&payload, recv_window))
        .body(payload)
        .send()
        .await
    {
        let body = res.text().await?;

        info!("Buy Futures Status {} = {}", &symbol, &body);
    } else {
        error!("Error in sending the futures order {}", symbol);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();
    let client = Client::new();
    let args = Command::parse();

    println!("Symbol: {}", args.symbol);
    println!("Quantity: {}", args.quantity);

    market_buy_futures_position(client, args.symbol, args.quantity, "5000").await?;
    Ok(())
}
