#![no_std]
#![no_main]

use feather_m4 as bsp;

use bsp::{
    ehal::blocking::delay::DelayUs,
    entry,
    hal::{clock::GenericClockController, delay::Delay, gpio::DynPin, prelude::*},
    pac::{CorePeripherals, Peripherals},
};
use panic_semihosting as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);

    let mut cols: [DynPin; 3] = [pins.a2.into(), pins.a0.into(), pins.a4.into()];
    let mut rows: [DynPin; 4] = [
        pins.a1.into(),
        pins.d0.into(),
        pins.a5.into(),
        pins.a3.into(),
    ];

    for row in rows.iter_mut() {
        row.into_pull_up_input();
    }

    let mut delayer = Delay::new(core.SYST, &mut clocks);

    const KEYS: &[&[char]] = &[
        &['1', '2', '3'],
        &['4', '5', '6'],
        &['7', '8', '9'],
        &['*', '0', '#'],
    ];

    let last_value: &mut [&mut [bool]] = &mut [
        &mut [false, false, false],
        &mut [false, false, false],
        &mut [false, false, false],
        &mut [false, false, false],
    ];

    loop {
        for (row_index, row) in rows.iter_mut().enumerate() {
            row.into_push_pull_output();
            row.set_low().ok();
            delayer.delay_us(50u8);
            for (col_index, col) in cols.iter_mut().enumerate() {
                col.into_pull_up_input();
                let col_value = col.is_low().unwrap_or_else(|_| {
                    rprintln!("is_low failed");
                    false
                });
                if col_value != last_value[row_index][col_index] {
                    rprintln!(
                        "row {} col {} col_value {} value {:#?}",
                        row_index,
                        col_index,
                        col_value,
                        KEYS[row_index][col_index]
                    );
                    last_value[row_index][col_index] = col_value;
                }
            }
            row.into_pull_up_input();
        }
    }
}
