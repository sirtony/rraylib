use rraylib::graphics::Color;
use rraylib::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::default();
    let mut rl = rraylib::init(options)?;
    let mut window = rl.window()?;

    while !window.should_close() {
        let mut ctx = rl.begin_drawing()?;

        ctx.clear_background(Color::RAY_WHITE);
        ctx.draw_fps(10, 10);
    }

    window.close()?;

    Ok(())
}
