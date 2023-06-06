use eyre::*;

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
