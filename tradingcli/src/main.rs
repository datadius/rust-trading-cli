mod web_utils;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    /// The symbol you want to trade
    symbol: String,
    /// The quantity of the symbol you want to trade
    quantity: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::parse();
    println!("Symbol: {}", args.symbol);
    println!("Quantity: {}", args.quantity);
    Ok(())
}
