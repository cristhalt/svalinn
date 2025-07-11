use aya::{
    Ebpf, include_bytes_aligned,
    programs::{Xdp, XdpFlags},
};
use svalinn_shared::rule::{MAX_RULES, XdpRule};

pub fn load_xdp(iface: String, rules: Vec<XdpRule>) -> Result<(), XdpError> {
    if rules.len() > MAX_RULES as usize {
        return Err(XdpError::TooMuchRules);
    }

    let mut ebpf = Ebpf::load(include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/svalinn-ebpf-xdp"
    )))?;

    let mut rules_map: aya::maps::Array<_, XdpRule> = aya::maps::Array::try_from(
        ebpf.map_mut("RULES")
            .expect("Failed to get eBPF XDP map for RULES"),
    )?;

    for (i, rule) in rules.into_iter().enumerate() {
        rules_map.set(i as u32, rule, 0)?;
    }

    let xdp: &mut Xdp = ebpf
        .program_mut("ebpf_main")
        .unwrap() // Can't fail
        .try_into()
        .unwrap(); // Can't fail

    xdp.attach(&iface, XdpFlags::default())
        .map_err(|err| XdpError::Attach(err, iface))?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum XdpError {
    #[error("Only {MAX_RULES} rules can be applied to each NIC")]
    TooMuchRules,
    #[error(transparent)]
    EbpfMap(#[from] aya::maps::MapError),
    #[error(transparent)]
    Ebpf(#[from] aya::EbpfError),
    #[error("Failed to attach to the interface {1}")]
    Attach(aya::programs::ProgramError, String),
}
