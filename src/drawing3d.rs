use crate::error::Error;
use crate::graphics::Drawing;
use crate::shapes::Shape3D;
use crate::sys::*;
use crate::{newtype, try_lock};
use std::sync::MutexGuard;

newtype!(VrStereoConfig, unload_vr_stereo_config);
newtype!(Model, unload_model);

pub trait Drawables3D {
    fn draw_shape<'t>(
        &mut self,
        shape: impl Into<Shape3D<'t>>,
        color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        match shape.into() {
            Shape3D::Line(start, end) => Ok(unsafe { draw_line_3_d(start, end, color.into()) }),
            Shape3D::Circle {
                center,
                radius,
                rotation_axis,
                rotation_angle,
            } => Ok(unsafe {
                draw_circle_3_d(center, radius, rotation_axis, rotation_angle, color.into())
            }),
            Shape3D::Triangle(v1, v2, v3) => {
                Ok(unsafe { draw_triangle_3_d(v1, v2, v3, color.into()) })
            }
            Shape3D::TriangleStrip(points) => {
                let ptr = points.as_ptr();
                Ok(unsafe { draw_triangle_strip_3_d(ptr, points.len() as i32, color.into()) })
            }
            Shape3D::Cube { position, size } => {
                Ok(unsafe { draw_cube_v(position, size, color.into()) })
            }
            Shape3D::Sphere { center, radius, .. } => {
                Ok(unsafe { draw_sphere(center, radius, color.into()) })
            }
            Shape3D::Cylinder {
                start_pos,
                end_pos,
                start_radius,
                end_radius,
                slices,
            } => Ok(unsafe {
                draw_cylinder_ex(
                    start_pos,
                    end_pos,
                    start_radius,
                    end_radius,
                    slices as i32,
                    color.into(),
                )
            }),
            Shape3D::Capsule {
                start_pos,
                end_pos,
                radius,
                slices,
                rings,
            } => Ok(unsafe {
                draw_capsule(
                    start_pos,
                    end_pos,
                    radius,
                    slices as i32,
                    rings as i32,
                    color.into(),
                )
            }),
            Shape3D::Plane { center, size } => {
                Ok(unsafe { draw_plane(center, size, color.into()) })
            }
            Shape3D::Grid { slices, spacing } => Ok(unsafe { draw_grid(slices as i32, spacing) }),
            Shape3D::Ray(ray) => Ok(unsafe { draw_ray(ray, color.into()) }),
        }
    }

    fn draw_wires<'t>(
        &mut self,
        shape: impl Into<Shape3D<'t>>,
        color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        match shape.into() {
            Shape3D::Line(start, end) => Ok(unsafe { draw_line_3_d(start, end, color.into()) }),
            Shape3D::Cube { position, size } => {
                Ok(unsafe { draw_cube_wires_v(position, size, color.into()) })
            }
            Shape3D::Sphere {
                center,
                radius,
                rings,
                slices,
            } => Ok(unsafe {
                draw_sphere_wires(center, radius, rings as i32, slices as i32, color.into())
            }),
            Shape3D::Cylinder {
                start_pos,
                end_pos,
                start_radius,
                end_radius,
                slices,
            } => Ok(unsafe {
                draw_cylinder_wires_ex(
                    start_pos,
                    end_pos,
                    start_radius,
                    end_radius,
                    slices as i32,
                    color.into(),
                )
            }),
            Shape3D::Capsule {
                start_pos,
                end_pos,
                radius,
                slices,
                rings,
            } => Ok(unsafe {
                draw_capsule_wires(
                    start_pos,
                    end_pos,
                    radius,
                    slices as i32,
                    rings as i32,
                    color.into(),
                )
            }),
            Shape3D::Grid { slices, spacing } => Ok(unsafe { draw_grid(slices as i32, spacing) }),
            Shape3D::Ray(ray) => Ok(unsafe { draw_ray(ray, color.into()) }),
            Shape3D::Circle { .. } => Err(Error::OperationNotSupported {
                verb: "wireframe drawing",
                noun: "circles",
            }),
            Shape3D::Triangle(_, _, _) => Err(Error::OperationNotSupported {
                verb: "wireframe drawing",
                noun: "triangles",
            }),
            Shape3D::TriangleStrip(_) => Err(Error::OperationNotSupported {
                verb: "wireframe drawing",
                noun: "triangle strips",
            }),
            Shape3D::Plane { .. } => Err(Error::OperationNotSupported {
                verb: "wireframe drawing",
                noun: "planes",
            }),
        }
    }
}

impl<'a> Drawing<'a> {
    pub fn begin_mode3d(&mut self, camera: &Camera3D) -> crate::error::Result<Drawing3D> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_mode_3d(*camera) }
        Ok(Drawing3D(guard))
    }

    pub fn draw3d<F>(&mut self, camera: &Camera3D, f: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut Drawing3D),
    {
        let mut ctx = self.begin_mode3d(camera)?;
        f(&mut ctx);
        Ok(())
    }

    pub fn begin_vr_stereo_mode(
        &mut self,
        config: VrStereoConfig,
    ) -> crate::error::Result<DrawingVr> {
        let guard = try_lock!(self.drawing).ok_or(Error::ThreadAlreadyLocked("drawing"))?;
        unsafe { begin_vr_stereo_mode(config.as_raw()) }
        Ok(DrawingVr(guard))
    }

    pub fn draw_vr<F>(&mut self, config: VrStereoConfig, f: F) -> crate::error::Result<()>
    where
        F: FnOnce(&mut DrawingVr),
    {
        let mut ctx = self.begin_vr_stereo_mode(config)?;
        f(&mut ctx);
        Ok(())
    }
}

#[allow(dead_code)]
pub struct Drawing3D<'a>(MutexGuard<'a, ()>);

impl<'a> Drawables3D for Drawing3D<'a> {}

impl Drop for Drawing3D<'_> {
    fn drop(&mut self) {
        unsafe { end_mode_3d() }
    }
}

#[allow(dead_code)]
pub struct DrawingVr<'a>(MutexGuard<'a, ()>);

impl<'a> Drawables3D for DrawingVr<'a> {}

impl Drop for DrawingVr<'_> {
    fn drop(&mut self) {
        unsafe { end_vr_stereo_mode() }
    }
}

impl Camera3D {
    pub fn position(&self, pos: Vector3) -> Self {
        Self {
            position: pos,
            ..*self
        }
    }

    pub fn positionf(&self, x: f32, y: f32, z: f32) -> Self {
        self.position(Vector3::new(x, y, z))
    }

    pub fn target(&self, target: Vector3) -> Self {
        Self { target, ..*self }
    }

    pub fn targetf(&self, x: f32, y: f32, z: f32) -> Self {
        self.target(Vector3::new(x, y, z))
    }

    pub fn up(&self, up: Vector3) -> Self {
        Self { up, ..*self }
    }

    pub fn upf(&self, x: f32, y: f32, z: f32) -> Self {
        self.up(Vector3::new(x, y, z))
    }

    pub fn fovy(&self, fovy: f32) -> Self {
        Self { fovy, ..*self }
    }

    pub fn projection(&self, projection: CameraProjection) -> Self {
        Self {
            projection: projection as i32,
            ..*self
        }
    }

    pub fn perspective(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Self {
        Self {
            position,
            target,
            up,
            fovy,
            projection: CameraProjection::Perspective as i32,
        }
    }

    pub fn orthographic(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Self {
        Self {
            position,
            target,
            up,
            fovy,
            projection: CameraProjection::Orthographic as i32,
        }
    }

    pub fn update(&mut self, mode: CameraMode) {
        unsafe { update_camera(self, mode as i32) }
    }

    pub fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        unsafe { update_camera_pro(self, movement, rotation, zoom) }
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self {
            position: Vector3::ZERO,
            target: Vector3::ZERO,
            up: Vector3::UP,
            fovy: 45.0,
            projection: CameraProjection::Perspective as i32,
        }
    }
}

impl Camera {
    pub fn screen_to_world(&self, position: Vector2) -> Ray {
        unsafe { get_screen_to_world_ray(position, *self) }
    }

    pub fn screen_to_world_viewport(&self, position: Vector2, width: u32, height: u32) -> Ray {
        unsafe { get_screen_to_world_ray_ex(position, *self, width as i32, height as i32) }
    }

    pub fn world_to_screen(&self, position: Vector3) -> Vector2 {
        unsafe { get_world_to_screen(position, *self) }
    }

    pub fn world_to_screen_viewport(&self, position: Vector3, width: u32, height: u32) -> Vector2 {
        unsafe { get_world_to_screen_ex(position, *self, width as i32, height as i32) }
    }

    pub fn matrix(&self) -> Matrix {
        unsafe { get_camera_matrix(*self) }
    }
}

impl Into<Matrix> for Camera {
    fn into(self) -> Matrix {
        unsafe { get_camera_matrix(self) }
    }
}
