use color_eyre::Result;
use douyudm::real_main;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    real_main().await
}
