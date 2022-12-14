use crate::maze::{Coord, Direction};
use core::fmt;
use embedded_graphics::{
    mono_font::{ascii::FONT_8X13_BOLD, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle},
    text::{Alignment, Text},
};
use heapless::String;

macro_rules! map_x_to_ratio {
    ($value:expr) => {
        $value / ORIGINAL_SCREEN_SIZE.width as f32
    };
}

macro_rules! map_y_to_ratio {
    ($value:expr) => {
        $value / ORIGINAL_SCREEN_SIZE.height as f32
    };
}

macro_rules! map_x_to_screen {
    ($value:expr) => {
        (SCREEN_SIZE.width as f32 * $value + 0.9) as i32
    };
}

macro_rules! map_y_to_screen {
    ($value:expr) => {
        (SCREEN_SIZE.height as f32 * $value + 0.9) as i32
    };
}

pub const ORIGINAL_SCREEN_SIZE: Size = Size::new(280, 192);

const ORIGINAL_FRONT_LEFT: f32 = 0.0;
const ORIGINAL_FRONT_TOP: f32 = 0.0;
const ORIGINAL_FRONT_RIGHT: f32 = map_x_to_ratio!(279.0);
const ORIGINAL_FRONT_BOTTOM: f32 = map_y_to_ratio!(159.0);

const ORIGINAL_BACK_LEFT: f32 = map_x_to_ratio!(69.0);
const ORIGINAL_BACK_TOP: f32 = map_y_to_ratio!(29.0);
const ORIGINAL_BACK_RIGHT: f32 = map_x_to_ratio!(209.0);
const ORIGINAL_BACK_BOTTOM: f32 = map_y_to_ratio!(129.0);

const ORIGINAL_LD_LEFT: f32 = map_x_to_ratio!(19.0);
const ORIGINAL_LD_RIGHT: f32 = map_x_to_ratio!(49.0);
const ORIGINAL_LRD_FRONT_TOP: f32 = map_y_to_ratio!(39.0);
const ORIGINAL_LRD_BACK_TOP: f32 = map_y_to_ratio!(49.0);
const ORIGINAL_LRD_FRONT_BOTTOM: f32 = map_y_to_ratio!(149.0);
const ORIGINAL_LRD_BACK_BOTTOM: f32 = map_y_to_ratio!(137.0);

const ORIGINAL_RD_LEFT: f32 = map_x_to_ratio!(259.0);
const ORIGINAL_RD_RIGHT: f32 = map_x_to_ratio!(229.0);

const ORIGINAL_TD_TOP: f32 = map_y_to_ratio!(9.0);
const ORIGINAL_TD_BOTTOM: f32 = map_y_to_ratio!(19.0);
const ORIGINAL_TBD_FRONT_LEFT: f32 = map_x_to_ratio!(109.0);
const ORIGINAL_TBD_BACK_LEFT: f32 = map_x_to_ratio!(119.0);
const ORIGINAL_TBD_FRONT_RIGHT: f32 = map_x_to_ratio!(169.0);
const ORIGINAL_TBD_BACK_RIGHT: f32 = map_x_to_ratio!(159.0);

const ORIGINAL_BD_TOP: f32 = map_y_to_ratio!(149.0);
const ORIGINAL_BD_BOTTOM: f32 = map_y_to_ratio!(139.0);

const ORIGINAL_FD_FRONT_LEFT: f32 = map_x_to_ratio!(119.0);
const ORIGINAL_FD_BACK_LEFT: f32 = map_x_to_ratio!(129.0);
const ORIGINAL_FD_FRONT_RIGHT: f32 = map_x_to_ratio!(159.0);
const ORIGINAL_FD_BACK_RIGHT: f32 = map_x_to_ratio!(149.0);
const ORIGINAL_FD_FRONT_TOP: f32 = map_y_to_ratio!(59.0);
const ORIGINAL_FD_FRONT_BOTTOM: f32 = map_y_to_ratio!(129.0);
const ORIGINAL_FD_BACK_TOP: f32 = map_y_to_ratio!(69.0);
const ORIGINAL_FD_BACK_BOTTOM: f32 = map_y_to_ratio!(119.0);

#[cfg(feature = "lcd_screen")]
pub const SCREEN_SIZE: Size = Size::new(320, 240);
#[cfg(feature = "memory_screen")]
pub const SCREEN_SIZE: Size = Size::new(400, 240);

const FRONT_LEFT: i32 = map_x_to_screen!(ORIGINAL_FRONT_LEFT);
const FRONT_RIGHT: i32 = map_x_to_screen!(ORIGINAL_FRONT_RIGHT);
const FRONT_TOP: i32 = map_y_to_screen!(ORIGINAL_FRONT_TOP);
const FRONT_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_FRONT_BOTTOM);
const BACK_LEFT: i32 = map_x_to_screen!(ORIGINAL_BACK_LEFT);
const BACK_RIGHT: i32 = map_x_to_screen!(ORIGINAL_BACK_RIGHT);
const BACK_TOP: i32 = map_y_to_screen!(ORIGINAL_BACK_TOP);
const BACK_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_BACK_BOTTOM);

const LD_LEFT: i32 = map_x_to_screen!(ORIGINAL_LD_LEFT);
const LD_RIGHT: i32 = map_x_to_screen!(ORIGINAL_LD_RIGHT);
const LRD_FRONT_TOP: i32 = map_y_to_screen!(ORIGINAL_LRD_FRONT_TOP);
const LRD_BACK_TOP: i32 = map_y_to_screen!(ORIGINAL_LRD_BACK_TOP);
const LRD_FRONT_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_LRD_FRONT_BOTTOM);
const LRD_BACK_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_LRD_BACK_BOTTOM);

const RD_LEFT: i32 = map_x_to_screen!(ORIGINAL_RD_LEFT);
const RD_RIGHT: i32 = map_x_to_screen!(ORIGINAL_RD_RIGHT);

const TD_TOP: i32 = map_y_to_screen!(ORIGINAL_TD_TOP);
const TD_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_TD_BOTTOM);
const TBD_FRONT_LEFT: i32 = map_x_to_screen!(ORIGINAL_TBD_FRONT_LEFT);
const TBD_BACK_LEFT: i32 = map_x_to_screen!(ORIGINAL_TBD_BACK_LEFT);
const TBD_FRONT_RIGHT: i32 = map_x_to_screen!(ORIGINAL_TBD_FRONT_RIGHT);
const TBD_BACK_RIGHT: i32 = map_x_to_screen!(ORIGINAL_TBD_BACK_RIGHT);

const BD_TOP: i32 = map_y_to_screen!(ORIGINAL_BD_TOP);
const BD_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_BD_BOTTOM);

const FD_FRONT_LEFT: i32 = map_x_to_screen!(ORIGINAL_FD_FRONT_LEFT);
const FD_BACK_LEFT: i32 = map_x_to_screen!(ORIGINAL_FD_BACK_LEFT);
const FD_FRONT_RIGHT: i32 = map_x_to_screen!(ORIGINAL_FD_FRONT_RIGHT);
const FD_BACK_RIGHT: i32 = map_x_to_screen!(ORIGINAL_FD_BACK_RIGHT);
const FD_FRONT_TOP: i32 = map_y_to_screen!(ORIGINAL_FD_FRONT_TOP);
const FD_FRONT_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_FD_FRONT_BOTTOM);
const FD_BACK_TOP: i32 = map_y_to_screen!(ORIGINAL_FD_BACK_TOP);
const FD_BACK_BOTTOM: i32 = map_y_to_screen!(ORIGINAL_FD_BACK_BOTTOM);

fn color_for_door(showing: bool) -> Rgb565 {
    if showing {
        Rgb565::BLACK
    } else {
        Rgb565::WHITE
    }
}

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

fn draw_lines_with_color<D>(
    points: &[Point],
    color: Rgb565,
    display: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let line_style = PrimitiveStyle::with_stroke(color, 1);
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

pub fn draw_room<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    const TOP_LEFT: Point = Point::new(FRONT_LEFT, FRONT_TOP);
    const TOP_RIGHT: Point = Point::new(FRONT_RIGHT, FRONT_TOP);
    const BOTTOM_LEFT: Point = Point::new(FRONT_LEFT, FRONT_BOTTOM);
    const BOTTOM_RIGHT: Point = Point::new(FRONT_RIGHT, FRONT_BOTTOM);

    const ROOM_OUTLINE: [Point; 10] = [
        TOP_LEFT,
        TOP_RIGHT,
        BOTTOM_RIGHT,
        BOTTOM_LEFT,
        TOP_LEFT,
        Point::new(BACK_LEFT, BACK_TOP),
        Point::new(BACK_RIGHT, BACK_TOP),
        Point::new(BACK_RIGHT, BACK_BOTTOM),
        Point::new(BACK_LEFT, BACK_BOTTOM),
        Point::new(BACK_LEFT, BACK_TOP),
    ];

    draw_lines(&ROOM_OUTLINE, display)?;

    const BOTTOM_RIGHT_LINE: [Point; 2] = [Point::new(BACK_RIGHT, BACK_TOP), TOP_RIGHT];
    draw_lines(&BOTTOM_RIGHT_LINE, display)?;

    const TOP_RIGHT_LINE: [Point; 2] = [Point::new(BACK_RIGHT, BACK_BOTTOM), BOTTOM_RIGHT];
    draw_lines(&TOP_RIGHT_LINE, display)?;

    const BOTTOM_LEFT_LINE: [Point; 2] = [Point::new(BACK_LEFT, BACK_BOTTOM), BOTTOM_LEFT];
    draw_lines(&BOTTOM_LEFT_LINE, display)
}

pub fn draw_left_door<D>(display: &mut D, showing: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let color = color_for_door(showing);

    const LD_FRAME: [Point; 4] = [
        Point::new(LD_LEFT, LRD_FRONT_BOTTOM),
        Point::new(LD_LEFT, LRD_FRONT_TOP),
        Point::new(LD_RIGHT, LRD_BACK_TOP),
        Point::new(LD_RIGHT, LRD_BACK_BOTTOM),
    ];
    draw_lines_with_color(&LD_FRAME, color, display)?;

    const LD_TOP: [Point; 2] = [
        Point::new(LD_LEFT, LRD_BACK_TOP),
        Point::new(LD_RIGHT, LRD_BACK_TOP),
    ];
    draw_lines_with_color(&LD_TOP, color, display)?;
    const LD_BOTTOM: [Point; 2] = [
        Point::new(LD_LEFT, LRD_BACK_BOTTOM),
        Point::new(LD_RIGHT, LRD_BACK_BOTTOM),
    ];
    draw_lines_with_color(&LD_BOTTOM, color, display)
}

pub fn draw_right_door<D>(display: &mut D, showing: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let color = color_for_door(showing);

    const RD_FRAME: [Point; 4] = [
        Point::new(RD_LEFT, LRD_FRONT_BOTTOM),
        Point::new(RD_LEFT, LRD_FRONT_TOP),
        Point::new(RD_RIGHT, LRD_BACK_TOP),
        Point::new(RD_RIGHT, LRD_BACK_BOTTOM),
    ];
    draw_lines_with_color(&RD_FRAME, color, display)?;

    const RD_TOP: [Point; 2] = [
        Point::new(RD_LEFT, LRD_BACK_TOP),
        Point::new(RD_RIGHT, LRD_BACK_TOP),
    ];
    draw_lines_with_color(&RD_TOP, color, display)?;
    const RD_BOTTOM: [Point; 2] = [
        Point::new(RD_LEFT, LRD_BACK_BOTTOM),
        Point::new(RD_RIGHT, LRD_BACK_BOTTOM),
    ];
    draw_lines_with_color(&RD_BOTTOM, color, display)
}

pub fn draw_top_door<D>(display: &mut D, showing: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let color = color_for_door(showing);

    const TD_FRAME: [Point; 5] = [
        Point::new(TBD_FRONT_LEFT, TD_TOP),
        Point::new(TBD_FRONT_RIGHT, TD_TOP),
        Point::new(TBD_BACK_RIGHT, TD_BOTTOM),
        Point::new(TBD_BACK_LEFT, TD_BOTTOM),
        Point::new(TBD_FRONT_LEFT, TD_TOP),
    ];
    draw_lines_with_color(&TD_FRAME, color, display)?;
    const TD_LEFT: [Point; 2] = [
        Point::new(TBD_BACK_LEFT, TD_TOP),
        Point::new(TBD_BACK_LEFT, TD_BOTTOM),
    ];
    draw_lines_with_color(&TD_LEFT, color, display)?;
    const TD_RIGHT: [Point; 2] = [
        Point::new(TBD_BACK_RIGHT, TD_TOP),
        Point::new(TBD_BACK_RIGHT, TD_BOTTOM),
    ];
    draw_lines_with_color(&TD_RIGHT, color, display)
}

pub fn draw_bottom_door<D>(display: &mut D, showing: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let color = color_for_door(showing);

    const BD_FRAME: [Point; 5] = [
        Point::new(TBD_FRONT_LEFT, BD_TOP),
        Point::new(TBD_FRONT_RIGHT, BD_TOP),
        Point::new(TBD_BACK_RIGHT, BD_BOTTOM),
        Point::new(TBD_BACK_LEFT, BD_BOTTOM),
        Point::new(TBD_FRONT_LEFT, BD_TOP),
    ];
    draw_lines_with_color(&BD_FRAME, color, display)?;
    const BD_LEFT: [Point; 2] = [
        Point::new(TBD_BACK_LEFT, BD_TOP),
        Point::new(TBD_BACK_LEFT, BD_BOTTOM),
    ];
    draw_lines_with_color(&BD_LEFT, color, display)?;
    const BD_RIGHT: [Point; 2] = [
        Point::new(TBD_BACK_RIGHT, BD_TOP),
        Point::new(TBD_BACK_RIGHT, BD_BOTTOM),
    ];
    draw_lines_with_color(&BD_RIGHT, color, display)
}

pub fn draw_front_door<D>(display: &mut D, showing: bool) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let color = color_for_door(showing);

    const FD_FRONT_FRAME: &[Point] = &[
        Point::new(FD_FRONT_LEFT, FD_FRONT_BOTTOM),
        Point::new(FD_FRONT_LEFT, FD_FRONT_TOP),
        Point::new(FD_FRONT_RIGHT, FD_FRONT_TOP),
        Point::new(FD_FRONT_RIGHT, FD_FRONT_BOTTOM),
    ];
    draw_lines_with_color(FD_FRONT_FRAME, color, display)?;

    const FD_BACK_FRAME: &[Point] = &[
        Point::new(FD_BACK_LEFT, FD_BACK_TOP),
        Point::new(FD_BACK_RIGHT, FD_BACK_TOP),
        Point::new(FD_BACK_RIGHT, FD_BACK_BOTTOM),
        Point::new(FD_BACK_LEFT, FD_BACK_BOTTOM),
        Point::new(FD_BACK_LEFT, FD_BACK_TOP),
    ];
    draw_lines_with_color(FD_BACK_FRAME, color, display)?;

    const FD_BOTTOM_LEFT: [Point; 2] = [
        Point::new(FD_FRONT_LEFT, FD_FRONT_BOTTOM),
        Point::new(FD_BACK_LEFT, FD_BACK_BOTTOM),
    ];
    draw_lines_with_color(&FD_BOTTOM_LEFT, color, display)?;

    const FD_BOTTOM_RIGHT: [Point; 2] = [
        Point::new(FD_FRONT_RIGHT, FD_FRONT_BOTTOM),
        Point::new(FD_BACK_RIGHT, FD_BACK_BOTTOM),
    ];
    draw_lines_with_color(&FD_BOTTOM_RIGHT, color, display)?;

    const FD_TOP_RIGHT: [Point; 2] = [
        Point::new(FD_FRONT_RIGHT, FD_FRONT_TOP),
        Point::new(FD_BACK_RIGHT, FD_BACK_TOP),
    ];
    draw_lines_with_color(&FD_TOP_RIGHT, color, display)?;

    const FD_TOP_LEFT: [Point; 2] = [
        Point::new(FD_FRONT_LEFT, FD_FRONT_TOP),
        Point::new(FD_BACK_LEFT, FD_BACK_TOP),
    ];
    draw_lines_with_color(&FD_TOP_LEFT, color, display)?;

    // Redraw the two pixels that might have been erased by the
    // left and right sides of the front door.
    const FIX_PIXELS: [Pixel<Rgb565>; 2] = [
        Pixel(Point::new(FD_FRONT_RIGHT, FD_FRONT_BOTTOM), Rgb565::BLACK),
        Pixel(Point::new(FD_FRONT_LEFT, FD_FRONT_BOTTOM), Rgb565::BLACK),
    ];
    display.draw_iter(FIX_PIXELS)
}

const STATUS_TOP: u32 = FRONT_BOTTOM as u32;
const STATUS_HEIGHT: u32 = SCREEN_SIZE.height - STATUS_TOP;
const STATUS_CENTER_V: i32 = (SCREEN_SIZE.height - STATUS_HEIGHT / 2 + 5) as i32;

pub fn draw_status<D>(
    display: &mut D,
    facing: Direction,
    position: Option<Coord>,
    hint: Option<Direction>,
    elapsed: u64,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();

    Rectangle::new(
        Point::new(0, FRONT_BOTTOM),
        Size::new(SCREEN_SIZE.width, STATUS_HEIGHT),
    )
    .into_styled(style)
    .draw(display)?;

    update_status(display, facing, position, hint, elapsed)?;

    Ok(())
}

pub fn update_status<D>(
    display: &mut D,
    facing: Direction,
    position: Option<Coord>,
    hint: Option<Direction>,
    elapsed: u64,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let style = MonoTextStyleBuilder::new()
        .font(&FONT_8X13_BOLD)
        .text_color(Rgb565::WHITE)
        .background_color(Rgb565::BLACK)
        .build();

    if let Some(hint) = hint {
        let mut label = String::<32>::new();
        let facing_str: &str = facing.into();
        let hint_str: &str = hint.into();
        fmt::write(&mut label, format_args!("{}[{}]", facing_str, hint_str)).expect("write");
        Text::with_alignment(
            &label,
            Point::new((SCREEN_SIZE.width / 2) as i32, STATUS_CENTER_V),
            style,
            Alignment::Center,
        )
        .draw(display)?;
    } else {
        Text::with_alignment(
            facing.into(),
            Point::new((SCREEN_SIZE.width / 2) as i32, STATUS_CENTER_V),
            style,
            Alignment::Center,
        )
        .draw(display)?;
    }

    update_time(display, elapsed)?;

    if let Some(position) = position {
        let mut label = String::<12>::new();
        fmt::write(
            &mut label,
            format_args!("{},{},{}", position.x, position.y, position.z),
        )
        .expect("format");
        Text::with_alignment(
            &label,
            Point::new((SCREEN_SIZE.width - 5) as i32, STATUS_CENTER_V),
            style,
            Alignment::Right,
        )
        .draw(display)?;
    }

    Ok(())
}

pub fn update_time<D>(display: &mut D, elapsed: u64) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let style = MonoTextStyleBuilder::new()
        .font(&FONT_8X13_BOLD)
        .text_color(Rgb565::WHITE)
        .background_color(Rgb565::BLACK)
        .build();

    let mut time_label = String::<32>::new();
    let seconds = (elapsed + 999) / 1000;
    fmt::write(
        &mut time_label,
        format_args!("Time: {:2}:{:02}", seconds / 60, seconds % 60),
    )
    .expect("write");
    let time = Text::with_alignment(
        &time_label,
        Point::new(5, STATUS_CENTER_V),
        style,
        Alignment::Left,
    );
    time.draw(display)?;

    Ok(())
}

pub fn draw_win<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let style = MonoTextStyle::new(&FONT_8X13_BOLD, Rgb565::WHITE);
    Text::with_alignment(
        "You Win!",
        Point::new(
            (SCREEN_SIZE.width / 2) as i32,
            (SCREEN_SIZE.height / 2) as i32,
        ),
        style,
        Alignment::Center,
    )
    .draw(display)?;
    Text::with_alignment(
        "Press any key to continue",
        Point::new(
            (SCREEN_SIZE.width / 2) as i32,
            (SCREEN_SIZE.height - 30) as i32,
        ),
        style,
        Alignment::Center,
    )
    .draw(display)?;
    Ok(())
}

pub fn draw_start<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let style = MonoTextStyle::new(&FONT_8X13_BOLD, Rgb565::WHITE);
    Text::with_alignment(
        "Press any key to start",
        Point::new(
            (SCREEN_SIZE.width / 2) as i32,
            (SCREEN_SIZE.height / 2) as i32,
        ),
        style,
        Alignment::Center,
    )
    .draw(display)?;
    Ok(())
}
