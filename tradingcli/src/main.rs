mod position_list;
mod web_utils;

use clap::Parser;
use log::{error, info};
use position_list::PositionList;
use reqwest::Client;
use std::error;
use web_utils::construct_headers;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct BuyFuturesPosition {
    /// m for market and l for limit
    order_type: String,
    /// The symbol you want to trade
    symbol: String,
    /// The quantity of the symbol you want to trade
    quantity: f32,
    /// The price you want to buy at
    price: Option<f32>,
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

async fn limit_buy_futures_position(
    client: Client,
    symbol: String,
    qty: f32,
    price: f32,
    recv_window: &str,
) -> Result<(), Box<dyn error::Error>> {
    let url = "https://api-testnet.bybit.com/v5/order/create";

    let payload = format!(
        r#"{{"category":"linear","symbol":"{}","side":"Buy","orderType":"Limit","qty":"{}","price":"{}"}}"#,
        symbol, qty, price
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

async fn get_leverage(
    client: Client,
    symbol: &str,
    recv_window: &str,
) -> Result<f32, Box<dyn error::Error>> {
    let params = format!("category=linear&symbol={}", symbol);
    let url = format!("https://api-testnet.bybit.com/v5/position/list?{}", params);
    let res = client
        .get(&url)
        .headers(construct_headers(&params, recv_window))
        .send()
        .await?;
    let body = res.text().await?;

    let leverage_json: PositionList = serde_json::from_str(&body).unwrap_or(PositionList {
        result: position_list::Result {
            list: vec![position_list::LeverageList {
                leverage: "0.0".to_string(),
            }],
        },
    });

    let value: f32 = leverage_json.result.list[0]
        .leverage
        .parse()
        .expect("Issue parsing the leverage to f32");

    Ok(value)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();
    let client = Client::new();
    let args = BuyFuturesPosition::parse();

    println!("Order Type: {}", args.order_type);
    println!("Symbol: {}", args.symbol);
    println!("Quantity: {}", args.quantity);
    println!("Price: {:?}", args.price);

    if args.order_type == "m" {
        market_buy_futures_position(client, args.symbol, args.quantity, "5000").await?;
    } else if args.order_type == "l" {
        limit_buy_futures_position(
            client,
            args.symbol,
            args.quantity,
            args.price.unwrap(),
            "5000",
        )
        .await?;
    }
    Ok(())
}
