use rraylib::graphics::{Camera3D, CameraMode, CameraProjection, Color};
use rraylib::math::{Vector2, Vector3};
use rraylib::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::builder().msaa(true).build();
    let mut rl = rraylib::init(options)?;
    let mut window = rl.window()?;

    let mut camera = Camera3D::default()
        .positionf(-7.0, 7.0, -7.0)
        .targetf(0.0, 0.0, 0.0)
        .fovy(70.0)
        .projection(CameraProjection::Perspective);

    while !window.should_close() {
        camera.update(CameraMode::Orbital);

        let mut ctx = rl.begin_drawing()?;
        ctx.clear_background(Color::SKY_BLUE);

        ctx.draw3d(&camera, move |ctx| {
            ctx.draw_plane(
                Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(32.0, 32.0),
                Color::LIGHT_GRAY,
            );
            ctx.draw_grid(32, 1.0);
            ctx.draw_cubev(
                Vector3::new(-1.0, 1.0, 1.0),
                Vector3::new(2.0, 2.0, 2.0),
                Color::RED,
            );
            ctx.draw_cubev(
                Vector3::new(-1.0, 1.0, -1.0),
                Vector3::new(2.0, 2.0, 2.0),
                Color::GREEN,
            );
            ctx.draw_cubev(
                Vector3::new(1.0, 1.0, 1.0),
                Vector3::new(2.0, 2.0, 2.0),
                Color::BLUE,
            );
            ctx.draw_cubev(
                Vector3::new(1.0, 1.0, -1.0),
                Vector3::new(2.0, 2.0, 2.0),
                Color::YELLOW,
            );
        })?;

        ctx.draw_fps(10, 10);
    }

    window.close()?;

    Ok(())
}
