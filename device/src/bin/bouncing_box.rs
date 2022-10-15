#![no_std]
#![no_main]

use feather_m4 as bsp;

use bsp::{
    entry,
    hal::{clock::GenericClockController, delay::Delay, prelude::*},
    pac::{CorePeripherals, Peripherals},
};
use display_interface_spi::SPIInterface;
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle, StyledDimensions},
};
use ili9341::{DisplaySize240x320, Ili9341, Orientation};
use panic_semihosting as _;
use rtt_target::{rprintln, rtt_init_print};

pub const SCREEN_SIZE: Size = Size::new(320, 240);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("spinning square");
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

    let sck = pins.sck;
    let miso = pins.miso;
    let mosi = pins.mosi;
    let mclk = &mut peripherals.MCLK;

    let lcd_dc = pins.d10.into_push_pull_output();
    let lcd_cs = pins.d9.into_push_pull_output();

    let sercom = peripherals.SERCOM1;
    let spi = bsp::spi_master(&mut clocks, 32.mhz(), sercom, mclk, sck, mosi, miso);
    let spi_iface = SPIInterface::new(spi, lcd_dc, lcd_cs);
    let reset_pin = pins.d12.into_push_pull_output();

    let mut delayer = Delay::new(core.SYST, &mut clocks);

    let mut lcd = Ili9341::new(
        spi_iface,
        reset_pin,
        &mut delayer,
        Orientation::Landscape,
        DisplaySize240x320,
    )
    .unwrap();

    lcd.clear(Rgb565::BLACK).expect("clear");

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::GREEN)
        .build();

    let bounds = Rectangle::new(Point::new(5, 5), Size::new(40, 40));
    let mut step = Point::new(1, 1);

    let mut rr = RoundedRectangle::with_equal_corners(bounds, Size::new(10, 10)).into_styled(style);

    rr.draw(&mut lcd).expect("RoundedRectangle");

    loop {
        let styled_bounds = rr.primitive.styled_bounding_box(&style);
        lcd.fill_solid(&styled_bounds, Rgb565::BLACK)
            .expect("fill_solid");
        rr.translate_mut(step);
        rr.draw(&mut lcd).expect("RoundedRectangle");

        let bottom_right = styled_bounds.bottom_right().expect("bottom_right");

        if styled_bounds.top_left.x <= 0 {
            step.x = 1;
        } else if bottom_right.x > 320 {
            step.x = -1;
        }

        if styled_bounds.top_left.y <= 0 {
            step.y = 1;
        } else if bottom_right.y > 240 {
            step.y = -1;
        }
    }
}
