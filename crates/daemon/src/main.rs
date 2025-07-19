use crate::{conf::GlobalConfig, logging::init_logging};

mod conf;
mod fw;
mod logging;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cfg = GlobalConfig::from_file()?;

    init_logging(cfg.log)?;

    Ok(())
}
