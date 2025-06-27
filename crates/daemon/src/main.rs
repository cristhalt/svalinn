use crate::{conf::GlobalConfig, logging::init_logging};

mod conf;
mod logging;

fn main() -> Result<(), anyhow::Error> {
    let cfg = GlobalConfig::from_file()?;

    init_logging(cfg.log)?;

    Ok(())
}
