#[cfg(feature = "std")]
use serde::Deserialize;

/// Represents the conditions that must be met for a firewall rule to match a packet.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct RuleCondition {
    /// IP-based filtering conditions.
    pub ip: Option<IpCondition>,
}

/// Specifies the traffic direction for classifier-based firewall rules.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub enum TrafficDirection {
    Inbound,
    Outbound,
    Both,
}

/// Defines IP address-based filtering conditions for firewall rules.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct IpCondition {
    pub src: Option<IpNet>,
    pub dst: Option<IpNet>,
}

/// Represents an IP network address with a prefix length.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct IpNet {
    /// The base IP address of the network range.
    /// Can be either IPv4 or IPv6 address as defined by `core::net::IpAddr`.
    pub addr: core::net::IpAddr,
    /// The prefix length in bits that defines the network mask.
    pub pfx_len: u8,
}
