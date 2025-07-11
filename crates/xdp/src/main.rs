#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    programs::XdpContext,
};
use svalinn_shared::rule::{MAX_RULES, XdpRule};

#[map]
static RULES: aya_ebpf::maps::Array<XdpRule> =
    aya_ebpf::maps::Array::with_max_entries(MAX_RULES, 0);

#[xdp]
fn ebpf_main(_ctx: XdpContext) -> u32 {
    xdp_action::XDP_PASS
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
