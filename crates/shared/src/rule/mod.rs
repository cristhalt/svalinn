#[cfg(feature = "std")]
use serde::Deserialize;

use crate::rule::condition::{RuleCondition, TrafficDirection};

mod condition;

/// Represents a XDP-based rule for high-performance but basic packet filtering at the network driver level.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct XdpRule {
    pub condition: RuleCondition,
    pub action: RuleAction,
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

#[cfg(feature = "std")]
unsafe impl aya::Pod for TcRule {}

/// Represents the action to do when a packet match the condition.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub enum RuleAction {
    Drop,
    Pass,
}
