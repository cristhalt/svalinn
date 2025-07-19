use serde::Deserialize;
use svalinn_shared::rule::{TcFirewallRule, XdpFirewallRule};

mod tc;
pub mod xdp;

/// Represents the complete firewall rules configuration.
#[derive(Debug, Deserialize, Default)]
pub struct FirewallRuleConfig {
    pub xdp: Vec<XdpFirewallRule>,
    pub tc: Vec<TcFirewallRule>,
}
