use anyhow::{Context, anyhow};
use aya_build::{build_ebpf, cargo_metadata::MetadataCommand};

fn main() -> Result<(), anyhow::Error> {
    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .context("Metadata command")?;

    let ebpf_package = metadata
        .packages
        .into_iter()
        .find(|package| package.name.as_str() == "svalinn-xdp")
        .ok_or_else(|| anyhow!("XDP eBPF package not found"))?;

    build_ebpf([ebpf_package]).context("eBPF build")?;

    Ok(())
}
