#![no_std]
#![no_main]

use feather_m4 as bsp;

use bsp::{
    ehal::blocking::delay::DelayMs,
    entry,
    hal::{
        clock::GenericClockController,
        delay::Delay,
        gpio::F,
        prelude::*,
        pwm::{Channel, TCC2Pinout, Tcc2Pwm},
    },
    pac::{CorePeripherals, Peripherals},
};
use panic_semihosting as _;
use quinti_maze::game::NOTES;
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

    let buzzer = pins.d4.into_alternate::<F>();
    let gclk0 = clocks.gclk0();

    let mut pwm = Tcc2Pwm::new(
        &clocks.tcc2_tcc3(&gclk0).unwrap(),
        440.hz(),
        peripherals.TCC2,
        TCC2Pinout::Pa14(buzzer),
        &mut peripherals.MCLK,
    );

    pwm.disable(Channel::_0);

    let mut delayer = Delay::new(core.SYST, &mut clocks);
    for (freq, duration, delay) in NOTES {
        rprintln!("freq, duration, delay = {}, {}, {}", freq, duration, delay);
        delayer.delay_ms(*delay as u32);
        pwm.set_period(freq.hz());
        let max_duty = pwm.get_max_duty();
        rprintln!("max_duty = {}", max_duty);
        pwm.set_duty(Channel::_0, max_duty / 2);
        pwm.enable(Channel::_0);
        delayer.delay_ms(*duration as u32);
        pwm.disable(Channel::_0);
    }
    pwm.disable(Channel::_0);

    loop {}
}
