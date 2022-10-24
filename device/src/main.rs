#![no_std]
#![no_main]

use debouncr::{debounce_stateful_3, DebouncerStateful, Edge, Repeat3};
use feather_m4 as bsp;
use panic_semihosting as _;

const KEYS: &[&[char]] = &[
    &['1', '2', '3'],
    &['4', '5', '6'],
    &['7', '8', '9'],
    &['*', '0', '#'],
];

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS_0, EVSYS_1, EVSYS_2])]
mod app {
    use super::*;
    use bsp::hal;
    use core::time::Duration;
    use display_interface_spi::SPIInterface;

    use hal::clock::GenericClockController;
    use hal::gpio::{DynPin, Pin, F, PA14};
    use hal::pac::Peripherals;
    use hal::prelude::*;
    use hal::pwm::{Channel, TCC2Pinout, Tcc2Pwm};

    use ili9341::{DisplaySize240x320, Ili9341, Orientation};

    use quinti_maze::game::{Command, Game, PlatformSpecific, NOTES};
    use rtt_target::{rprintln, rtt_init_print};
    use systick_monotonic::*;

    type KeyDebouncer = DebouncerStateful<u8, Repeat3>;

    #[derive(Default, Debug)]
    pub struct DevicePlatform;

    impl PlatformSpecific for DevicePlatform {
        fn play_victory_notes(&mut self) {
            play_note_with_delay::spawn(0).ok();
        }

        fn ticks(&mut self) -> u64 {
            monotonic_millis()
        }
    }

    pub fn delay_ms(ms: u32) {
        const CYCLES_PER_MILLIS: u32 = SYSCLK_HZ / 1000;
        cortex_m::asm::delay(CYCLES_PER_MILLIS.saturating_mul(ms));
    }

    pub fn delay_for(delay: Duration) {
        delay_ms(delay.as_millis() as u32)
    }

    const SYSCLK_HZ: u32 = 120_000_000;

    pub struct CycleDelay;

    impl Default for CycleDelay {
        fn default() -> CycleDelay {
            CycleDelay
        }
    }

    impl embedded_hal::blocking::delay::DelayMs<u16> for CycleDelay {
        fn delay_ms(&mut self, ms: u16) {
            delay_ms(ms.into())
        }
    }

    fn monotonic_millis() -> u64 {
        app::monotonics::now().duration_since_epoch().to_millis()
    }

    type LcdCsPin = Pin<hal::gpio::PA20, hal::gpio::Output<hal::gpio::PushPull>>;
    type LcdDcPin = Pin<hal::gpio::PA19, hal::gpio::Output<hal::gpio::PushPull>>;
    type LcdResetPin = Pin<hal::gpio::PA22, hal::gpio::Output<hal::gpio::PushPull>>;

    #[local]
    struct Local {
        lcd: Ili9341<SPIInterface<bsp::Spi, LcdCsPin, LcdDcPin>, LcdResetPin>,
        cols: [DynPin; 3],
        rows: [DynPin; 4],
        debouncers: [KeyDebouncer; 12],
    }

    #[shared]
    struct Shared {
        game: Game<DevicePlatform>,
        pwm: Tcc2Pwm<PA14, hal::gpio::Alternate<F>>,
    }

    #[monotonic(binds = SysTick, default = true)]
    type RtcMonotonic = Systick<100>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        // Set up clock that drives RTIC
        let mono = Systick::new(cx.core.SYST, 120_000_000);

        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.PORT);

        // Initialize MC clocks
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
        );

        let gclk0 = clocks.gclk0();

        // Set up LCD driver
        let sck = pins.sck;
        let miso = pins.miso;
        let mosi = pins.mosi;
        let mclk = &mut peripherals.MCLK;

        let lcd_dc = pins.d10.into_push_pull_output();
        let lcd_cs = pins.d9.into_push_pull_output();

        let sercom = peripherals.SERCOM1;
        let spi = bsp::spi_master(&mut clocks, 4.mhz(), sercom, mclk, sck, mosi, miso);
        let spi_iface = SPIInterface::new(spi, lcd_dc, lcd_cs);
        let reset_pin = pins.d12.into_push_pull_output();

        let lcd = Ili9341::new(
            spi_iface,
            reset_pin,
            &mut CycleDelay::default(),
            Orientation::Landscape,
            DisplaySize240x320,
        )
        .unwrap();

        // Set up the GPIOs for scanning the matrix keypad
        let cols = [pins.a2.into(), pins.a0.into(), pins.a4.into()];
        let mut rows: [DynPin; 4] = [
            pins.a1.into(),
            pins.d0.into(),
            pins.a5.into(),
            pins.a3.into(),
        ];

        for row in rows.iter_mut() {
            row.into_pull_up_input();
        }

        let debouncers: [KeyDebouncer; 12] = [
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

        // Create the game
        let game = Game::new();

        // start the keypad spawning task
        scan::spawn().unwrap();

        // Start the render task
        render_game::spawn().unwrap();

        (
            Shared { game, pwm },
            Local {
                lcd,
                cols,
                rows,
                debouncers,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [lcd], shared = [game])]
    fn render_game(mut cx: render_game::Context) {
        cx.shared.game.lock(|game| {
            if let Err(e) = game.draw(cx.local.lcd) {
                rprintln!("err = {:?}", e);
            }
        });
        render_game::spawn_after(500.millis()).ok();
    }

    #[task(priority = 1, local = [rows, cols, debouncers], shared = [game])]
    fn scan(mut cx: scan::Context) {
        for (row_index, row) in cx.local.rows.iter_mut().enumerate() {
            row.into_push_pull_output();
            row.set_low().ok();
            delay_ms(1);
            for (col_index, col) in cx.local.cols.iter_mut().enumerate() {
                let index = row_index * 3 + col_index;
                col.into_pull_up_input();
                let col_value = col.is_low().unwrap_or_else(|_| {
                    rprintln!("is_low failed");
                    false
                });
                let edge = cx.local.debouncers[index].update(col_value);
                if Some(Edge::Rising) == edge {
                    let more_processing = cx.shared.game.lock(|game| game.key_hit());
                    if more_processing {
                        match KEYS[row_index][col_index] {
                            '1' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::MoveDown)),
                            '2' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::MoveForward)),
                            '3' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::MoveUp)),
                            '4' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::MoveLeft)),
                            '6' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::MoveRight)),
                            '7' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::TurnLeft)),
                            '9' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::TurnRight)),
                            '*' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::ToggleShowPosition)),
                            '#' => cx
                                .shared
                                .game
                                .lock(|game| game.handle_command(Command::ShowHints)),
                            _ => (),
                        }
                    }
                }
            }
            row.into_pull_up_input();
        }
        scan::spawn_after(10.millis()).ok();
    }

    #[task(priority = 2)]
    fn play_note_with_delay(_cx: play_note_with_delay::Context, index: usize) {
        if index < NOTES.len() {
            let note = &NOTES[index];
            if note.delay > 0 {
                start_note::spawn_after(note.delay.millis(), index).ok();
            } else {
                start_note::spawn(index).ok();
            }
        }
    }

    #[task(priority = 2, shared = [pwm])]
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

    #[task(priority = 2, shared = [pwm])]
    fn end_note(mut cx: end_note::Context, index: usize) {
        cx.shared.pwm.lock(|pwm| {
            pwm.disable(Channel::_0);
        });
        play_note_with_delay::spawn_after(10.millis(), index + 1).ok();
    }
}
