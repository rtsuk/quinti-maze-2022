#![no_std]
#![no_main]

use feather_m4 as hal;
use hal::entry;
use panic_semihosting as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    loop {}
}
