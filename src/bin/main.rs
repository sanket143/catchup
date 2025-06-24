use anyhow::Result;
use ketchup::{migrate, run};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.last().unwrap() == "migrate" {
        migrate().await?;
        return Ok(());
    }

    run().await?;

    Ok(())
}
