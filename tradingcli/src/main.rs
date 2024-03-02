use clap::Parser;

#[derive(Parser)]
struct Command {
    symbol: String,
    quantity: f32,
}

fn main() {
    let args = Command::parse();
    println!("Symbol: {}", args.symbol);
    println!("Quantity: {}", args.quantity);
}
