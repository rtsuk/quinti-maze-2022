#![no_std]
#![no_main]

use feather_m4 as bsp;

use bsp::{
    ehal::blocking::delay::DelayMs,
    entry,
    hal::{clock::GenericClockController, delay::Delay, gpio::DynPin, prelude::*},
    pac::{CorePeripherals, Peripherals},
};
use debouncr::{debounce_stateful_3, DebouncerStateful, Edge, Repeat3};
use panic_semihosting as _;
use rtt_target::{rprintln, rtt_init_print};

type KeyDebouncer = DebouncerStateful<u8, Repeat3>;

const KEYS: &[&[char]] = &[
    &['1', '2', '3'],
    &['4', '5', '6'],
    &['7', '8', '9'],
    &['*', '0', '#'],
];

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

    let mut debouncers: [KeyDebouncer; 12] = [
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
        debounce_stateful_3(false),
    ];

    let mut delayer = Delay::new(core.SYST, &mut clocks);

    loop {
        for (row_index, row) in rows.iter_mut().enumerate() {
            row.into_push_pull_output();
            row.set_low().ok();
            delayer.delay_ms(1u8);
            for (col_index, col) in cols.iter_mut().enumerate() {
                let index = row_index * 3 + col_index;
                col.into_pull_up_input();
                let col_value = col.is_low().unwrap_or_else(|_| {
                    rprintln!("is_low failed");
                    false
                });
                let edge = debouncers[index].update(col_value);
                if Some(Edge::Rising) == edge {
                    rprintln!(
                        "row {} col {} value {:#?}",
                        row_index,
                        col_index,
                        KEYS[row_index][col_index]
                    );
                }
            }
            row.into_pull_up_input();
        }
    }
}
