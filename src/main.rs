#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use cortex_m_rt::entry;
use atsamd_hal as _;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    loop {
        // Your code goes here
    }
}
