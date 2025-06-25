use anyhow::Result;
use ketchup::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
