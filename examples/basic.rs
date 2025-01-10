use rraylib::graphics::{Color, Drawables2D, Font};
use rraylib::math::Vector2;
use rraylib::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::default();
    let rl = rraylib::init(options)?;
    let window = rl.window()?;

    const TEXT: &str = "Woohoo! You're using Raylib from safe Rust!";
    const FONT_SIZE: f32 = 20.0;
    const FONT_SPACING: f32 = 1.0;

    let font = Font::default();

    while !window.close_requested() {
        let mut ctx = rl.begin_drawing()?;

        ctx.clear_background(Color::RAY_WHITE);

        let sz = ctx.measure_text_ex(&font, TEXT, FONT_SIZE, FONT_SPACING);
        let center = Vector2::new(
            window.render_width() as f32 / 2.0 - sz.x / 2.0,
            window.render_height() as f32 / 2.0 - sz.y / 2.0,
        );

        ctx.draw_text_ex(
            &font,
            TEXT,
            center,
            FONT_SIZE,
            FONT_SPACING,
            Color::DARK_GRAY,
        )?;

        ctx.draw_fps(10, 10);
    }

    window.close()?;

    Ok(())
}
