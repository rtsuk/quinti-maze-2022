#![no_std]
#![no_main]

use feather_m4 as bsp;

use panic_semihosting as _;

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS_0])]
mod app {
    use super::*;
    use bsp::hal;
    use hal::clock::GenericClockController;
    use hal::gpio::{F, PA14};
    use hal::pac::Peripherals;
    use hal::prelude::*;
    use hal::pwm::{Channel, TCC2Pinout, Tcc2Pwm};
    use quinti_maze::game::NOTES;
    use rtt_target::{rprintln, rtt_init_print};
    use systick_monotonic::*;

    #[local]
    struct Local {}

    #[shared]
    struct Shared {
        pwm: Tcc2Pwm<PA14, hal::gpio::Alternate<F>>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type RtcMonotonic = Systick<100>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        let mono = Systick::new(cx.core.SYST, 120_000_000);
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.PORT);

        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
        );

        let gclk0 = clocks.gclk0();

        // Set up the peizo speaker
        let buzzer = pins.d4.into_alternate::<F>();

        let mut pwm = Tcc2Pwm::new(
            &clocks.tcc2_tcc3(&gclk0).unwrap(),
            440.hz(),
            peripherals.TCC2,
            TCC2Pinout::Pa14(buzzer),
            &mut peripherals.MCLK,
        );

        let max_duty = pwm.get_max_duty();
        pwm.set_duty(Channel::_0, max_duty / 4);
        pwm.disable(Channel::_0);

        // Start the blink task
        play_note_with_delay::spawn(0).ok();

        (Shared { pwm }, Local {}, init::Monotonics(mono))
    }

    #[task]
    fn play_note_with_delay(_cx: play_note_with_delay::Context, index: usize) {
        if index < NOTES.len() {
            let note = &NOTES[index];
            rprintln!("play_note_with_delay {}:{:#?}", index, note);
            if note.delay > 0 {
                start_note::spawn_after(note.delay.millis(), index).ok();
            } else {
                start_note::spawn(index).ok();
            }
        }
    }

    #[task(shared = [pwm])]
    fn start_note(mut cx: start_note::Context, index: usize) {
        if index < NOTES.len() {
            let note = &NOTES[index];
            cx.shared.pwm.lock(|pwm| {
                pwm.set_period((note.frequency as u32).hz());
                let max_duty = pwm.get_max_duty();
                pwm.set_duty(Channel::_0, max_duty / 2);
                pwm.enable(Channel::_0);
            });
            end_note::spawn_after((note.duration as u64).millis(), index).ok();
        }
    }

    #[task(shared = [pwm])]
    fn end_note(mut cx: end_note::Context, index: usize) {
        cx.shared.pwm.lock(|pwm| {
            pwm.disable(Channel::_0);
        });
        play_note_with_delay::spawn_after(10.millis(), index + 1).ok();
    }
}
