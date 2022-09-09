//! Uses RTIC with the RTC as time source to blink an LED.
//!
//! The idle task is sleeping the CPU, so in practice this gives similar power
//! figure as the "sleeping_timer_rtc" example.
#![no_std]
#![no_main]

use feather_m4 as bsp;

use panic_semihosting as _;

#[rtic::app(device = bsp::pac, peripherals = true, dispatchers = [EVSYS_0])]
mod app {
    use super::*;
    use bsp::{hal, pin_alias};
    use display_interface_spi::SPIInterface;
    use hal::clock::GenericClockController;
    use hal::gpio::Pin;
    use hal::pac::Peripherals;
    use hal::prelude::*;
    use ili9341::{DisplaySize240x320, Ili9341, Orientation};
    use quinti_maze::{game::Game, maze::MazeGenerator};
    use rtt_target::{rprintln, rtt_init_print};
    use systick_monotonic::*;

    /// Worlds worst delay function.
    #[inline(always)]
    pub fn delay_ms(ms: u32) {
        const CYCLES_PER_MILLIS: u32 = SYSCLK_HZ / 1000;
        cortex_m::asm::delay(CYCLES_PER_MILLIS.saturating_mul(ms));
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
        app::monotonics::now()
            .duration_since_epoch()
            .to_millis()
            .try_into()
            .unwrap_or(0)
    }

    type LcdCsPin = Pin<hal::gpio::PA20, hal::gpio::Output<hal::gpio::PushPull>>;
    type LcdDcPin = Pin<hal::gpio::PA19, hal::gpio::Output<hal::gpio::PushPull>>;
    type LcdResetPin = Pin<hal::gpio::PA22, hal::gpio::Output<hal::gpio::PushPull>>;

    #[local]
    struct Local {
        red_led: bsp::RedLed,
        game: Game,
        lcd: Ili9341<SPIInterface<bsp::Spi, LcdCsPin, LcdDcPin>, LcdResetPin>,
    }

    #[shared]
    struct Shared {}

    #[monotonic(binds = SysTick, default = true)]
    type RtcMonotonic = Systick<100>;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        delay_ms(100);

        let _ = monotonic_millis();

        let mono = Systick::new(cx.core.SYST, 120_000_000);
        let mut peripherals: Peripherals = cx.device;
        let pins = bsp::Pins::new(peripherals.PORT);
        let red_led: bsp::RedLed = pin_alias!(pins.red_led).into();

        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
        );

        let sck = pins.sck;
        let miso = pins.miso;
        let mosi = pins.mosi;
        let mclk = &mut peripherals.MCLK;

        let lcd_dc = pins.d10.into_push_pull_output();
        let lcd_cs = pins.d9.into_push_pull_output();

        // Start the blink task
        blink::spawn().unwrap();

        let sercom = peripherals.SERCOM1;
        let spi = bsp::spi_master(&mut clocks, 8.mhz(), sercom, mclk, sck, mosi, miso);
        let spi_iface = SPIInterface::new(spi, lcd_dc, lcd_cs);
        let reset_pin = pins.d12.into_push_pull_output();

        let mut lcd = Ili9341::new(
            spi_iface,
            reset_pin,
            &mut CycleDelay::default(),
            Orientation::Landscape,
            DisplaySize240x320,
        )
        .unwrap();

        let mut generator = MazeGenerator::default();
        generator.generate(Some(13));
        let maze = generator.take();

        let game = Game::new(maze);

        game.draw(&mut lcd, 0).expect("draw");

        (
            Shared {},
            Local { red_led, game, lcd },
            init::Monotonics(mono),
        )
    }

    #[task(local = [game, lcd, red_led])]
    fn blink(cx: blink::Context) {
        let time = monotonic_millis();
        if let Err(e) =  cx.local.game.draw(cx.local.lcd, time) {
            rprintln!("err = {:?}", e);
        }
        cx.local.red_led.toggle().unwrap();
        blink::spawn_after(500.millis()).ok();
    }
}
