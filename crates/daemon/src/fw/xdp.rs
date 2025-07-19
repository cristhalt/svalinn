use std::collections::HashMap;

use aya::{
    Ebpf, include_bytes_aligned,
    programs::{Xdp, XdpFlags},
};
use svalinn_shared::rule::{MAX_RULES, XdpFirewallRule, XdpRule};

#[derive(Debug)]
pub struct XdpManager {
    ebpfs: HashMap<String, Ebpf>,
    rules: HashMap<String, Vec<XdpRule>>,
}

impl XdpManager {
    pub fn new() -> Self {
        Self {
            ebpfs: HashMap::new(),
            rules: HashMap::new(),
        }
    }

    pub fn apply_rules(&mut self, rules: Vec<XdpFirewallRule>) -> Result<(), XdpError> {
        self.rules = rules.into_iter().fold(HashMap::new(), |mut acc, fw_rule| {
            acc.entry(fw_rule.iif)
                .or_insert_with(Vec::new)
                .push(fw_rule.rule);
            acc
        });

        self.load_xdp()?;

        for (iif, rules) in &self.rules {
            if rules.len() > MAX_RULES as usize {
                return Err(XdpError::TooMuchRules);
            }

            if let Some(ebpf) = self.ebpfs.get_mut(iif) {
                let mut rules_map: aya::maps::Array<_, XdpRule> = aya::maps::Array::try_from(
                    ebpf.map_mut("RULES")
                        .expect("Failed to get eBPF XDP map for RULES"),
                )?;

                for (i, rule) in rules.iter().enumerate() {
                    rules_map.set(i as u32, rule.clone(), 0)?;
                }
            }
        }

        Ok(())
    }

    fn load_xdp(&mut self) -> Result<(), XdpError> {
        for (iif, _rules) in &self.rules {
            if self.ebpfs.get(iif).is_none() {
                let mut ebpf = Ebpf::load(include_bytes_aligned!(concat!(
                    env!("OUT_DIR"),
                    "/svalinn-ebpf-xdp"
                )))?;

                let xdp: &mut Xdp = ebpf
                    .program_mut("ebpf_main")
                    .unwrap() // Can't fail
                    .try_into()
                    .unwrap(); // Can't fail

                xdp.load().map_err(|err| XdpError::Load(err, iif.clone()))?;
                xdp.attach(iif, XdpFlags::default())
                    .map_err(|err| XdpError::Load(err, iif.clone()))?;

                self.ebpfs.insert(iif.clone(), ebpf);
            }
        }

        Ok(())
    }
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
    Load(aya::programs::ProgramError, String),
}
