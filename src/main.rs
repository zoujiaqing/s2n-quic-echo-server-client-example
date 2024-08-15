mod server;
mod client;

use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [server|client]", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "server" => server::run().await?,
        "client" => client::run().await?,
        _ => eprintln!("Invalid argument. Use 'server' or 'client'."),
    }

    Ok(())
}
