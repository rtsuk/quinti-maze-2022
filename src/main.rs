#![no_std]
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

fn draw_lines<D>(points: &[Point], display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let line_style = PrimitiveStyle::with_stroke(Rgb565::BLACK, 1);
    let mut last_point = None;

    for point in points {
        if let Some(last_point) = last_point.as_ref() {
            Line::new(*last_point, *point)
                .into_styled(line_style)
                .draw(display)?;
        }
        last_point = Some(*point);
    }

    Ok(())
}

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

const IWIDTH: i32 = WIDTH as i32;
const IHEIGHT: i32 = HEIGHT as i32;

const INNER_WIDTH : i32 = IWIDTH / 2;
const INNER_HEIGHT : i32 = IHEIGHT / 2;

const BOTTOM: i32 = IHEIGHT - 1;
const RIGHT: i32 = IWIDTH - 1;

const INNER_LEFT: i32 = (IWIDTH - INNER_WIDTH) / 2;
const INNER_TOP: i32 = (IHEIGHT - INNER_HEIGHT) / 2;
const INNER_RIGHT: i32 = IWIDTH - INNER_LEFT;
const INNER_BOTTOM: i32 = IHEIGHT - INNER_TOP;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(WIDTH, HEIGHT));

    display.clear(Rgb565::WHITE)?;

    const TOP_LEFT: Point = Point::new(0, 0);
    const TOP_RIGHT: Point = Point::new(RIGHT, 0);
    const BOTTOM_LEFT: Point = Point::new(0, BOTTOM);
    const BOTTOM_RIGHT: Point = Point::new(RIGHT, BOTTOM);

    const ROOM_OUTLINE: [Point; 10] = [
        TOP_LEFT,
        TOP_RIGHT,
        BOTTOM_RIGHT,
        BOTTOM_LEFT,
        TOP_LEFT,
        Point::new(INNER_LEFT, INNER_TOP),
        Point::new(INNER_RIGHT, INNER_TOP),
        Point::new(INNER_RIGHT, INNER_BOTTOM),
        Point::new(INNER_LEFT, INNER_BOTTOM),
        Point::new(INNER_LEFT, INNER_TOP),
    ];

    draw_lines(&ROOM_OUTLINE, &mut display)?;

    const BOTTOM_RIGHT_LINE: [Point; 2] = [Point::new(INNER_RIGHT, INNER_TOP), TOP_RIGHT];
    draw_lines(&BOTTOM_RIGHT_LINE, &mut display)?;

    const TOP_RIGHT_LINE: [Point; 2] = [Point::new(INNER_RIGHT, INNER_BOTTOM), BOTTOM_RIGHT];
    draw_lines(&TOP_RIGHT_LINE, &mut display)?;

    const BOTTOM_LEFT_LINE: [Point; 2] = [Point::new(INNER_LEFT, INNER_BOTTOM), BOTTOM_LEFT];
    draw_lines(&BOTTOM_LEFT_LINE, &mut display)?;

    let output_settings = OutputSettings::default();
    Window::new("Quinti-Maze", &output_settings).show_static(&display);

    Ok(())
}
