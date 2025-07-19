#[cfg(feature = "std")]
use serde::Deserialize;

use crate::rule::condition::{RuleCondition, TrafficDirection};

mod condition;

/// The maximum number of rule that can be applied per NIC.
pub const MAX_RULES: u32 = 1024;

/// Represents a XDP-based rule for high-performance but basic packet filtering at the network driver level.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct XdpRule {
    pub condition: RuleCondition,
    pub action: RuleAction,
}

/// Represents a XDP rule for the user-land side.
#[cfg(feature = "std")]
#[derive(Debug, Deserialize)]
pub struct XdpFirewallRule {
    pub iif: String,
    #[serde(flatten)]
    pub rule: XdpRule,
}

#[cfg(feature = "std")]
unsafe impl aya::Pod for XdpRule {}

/// Represents a TC Classifier rule for traffic control and more complex filtering scenarios.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct TcRule {
    pub condition: RuleCondition,
    pub direction: TrafficDirection,
    pub action: RuleAction,
}

/// Represents a TC rule for the user-land side.
#[cfg(feature = "std")]
#[derive(Debug, Deserialize)]
pub struct TcFirewallRule {
    /// The interfaces to apply the rule, if the rule direction is `TrafficDirection::Inbound`.
    ///
    /// If empty, the rule will be applied to all interfaces.
    pub iif: Option<String>,
    /// The interfaces to apply the rule, if the rule direction is `TrafficDirection::Outbound`.
    ///
    /// If empty, the rule will be applied to all interfaces.
    pub oif: Option<String>,
    #[serde(flatten)]
    pub rule: TcRule,
}

#[cfg(feature = "std")]
unsafe impl aya::Pod for TcRule {}

/// Represents the action to do when a packet match the condition.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub enum RuleAction {
    Drop,
    Pass,
}
