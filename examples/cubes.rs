use rraylib::graphics::{Camera3D, CameraMode, CameraProjection, Color, Drawables2D, Shape3D};
use rraylib::math::{Vector2, Vector3};
use rraylib::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::builder().msaa(true).build();
    let rl = rraylib::init(options)?;
    let window = rl.window()?;

    let mut camera = Camera3D::default()
        .positionf(-7.0, 7.0, -7.0)
        .targetf(0.0, 0.0, 0.0)
        .fovy(70.0)
        .projection(CameraProjection::Perspective);

    let plane = Shape3D::Plane {
        center: Vector3::ZERO,
        size: Vector2::new(32.0, 32.0),
    };

    let grid = Shape3D::Grid {
        slices: 32,
        spacing: 1.0,
    };

    let red_cube = Shape3D::Cube {
        position: Vector3::new(-1.0, 1.0, 1.0),
        size: Vector3::new(2.0, 2.0, 2.0),
    };

    let green_cube = Shape3D::Cube {
        position: Vector3::new(-1.0, 1.0, -1.0),
        size: Vector3::new(2.0, 2.0, 2.0),
    };

    let blue_cube = Shape3D::Cube {
        position: Vector3::new(1.0, 1.0, 1.0),
        size: Vector3::new(2.0, 2.0, 2.0),
    };

    let yellow_cube = Shape3D::Cube {
        position: Vector3::new(1.0, 1.0, -1.0),
        size: Vector3::new(2.0, 2.0, 2.0),
    };

    while !window.should_close() {
        camera.update(CameraMode::Orbital);

        let mut ctx = rl.begin_drawing()?;
        ctx.clear_background(Color::SKY_BLUE);

        let mut three = ctx.begin_mode3d(&camera)?;
        three.draw_shape(plane, Color::LIGHT_GRAY)?;
        three.draw_shape(grid, Color::DARK_GRAY)?;
        three.draw_shape(red_cube, Color::RED)?;
        three.draw_shape(green_cube, Color::GREEN)?;
        three.draw_shape(blue_cube, Color::BLUE)?;
        three.draw_shape(yellow_cube, Color::YELLOW)?;
        std::mem::drop(three);

        ctx.draw_fps(10, 10);
    }

    window.close()?;

    Ok(())
}
