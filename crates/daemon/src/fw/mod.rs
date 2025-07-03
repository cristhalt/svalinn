use serde::Deserialize;
use svalinn_shared::rule::{TcRule, XdpRule};

mod tc;
mod xdp;

/// Represents the complete firewall rules configuration.
#[derive(Debug, Deserialize, Default)]
pub struct FirewallRules {
    pub xdp: Vec<XdpRule>,
    pub tc: Vec<TcRule>,
}
