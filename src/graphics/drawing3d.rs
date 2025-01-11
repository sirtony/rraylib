#![allow(clippy::too_many_arguments)]

use crate::error::{Error, Result};
use crate::graphics::drawing2d::Texture;
use crate::graphics::shapes::Shape3D;
use crate::graphics::Drawing;
use crate::graphics::Image;
use crate::sys::*;
use crate::{newtype, try_lock};
use std::ffi::CString;
use std::path::Path;
use std::ptr::{addr_of, addr_of_mut};
use std::sync::MutexGuard;

newtype!(VrStereoConfig, @unload_vr_stereo_config);
newtype!(Model, @unload_model);
newtype!(Mesh, @unload_mesh);
newtype!(Material, @unload_material);
newtype!(ModelAnimation, @unload_model_animation);

pub trait Drawables3D {
    fn draw_shape<'t>(
        &mut self,
        shape: impl Into<Shape3D<'t>>,
        color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        match shape.into() {
            Shape3D::Line(start, end) => {
                unsafe { draw_line_3d(start, end, color.into()) };
                Ok(())
            }
            Shape3D::Circle {
                center,
                radius,
                rotation_axis,
                rotation_angle,
            } => {
                unsafe {
                    draw_circle_3d(center, radius, rotation_axis, rotation_angle, color.into())
                };
                Ok(())
            }
            Shape3D::Triangle(v1, v2, v3) => {
                unsafe { draw_triangle_3d(v1, v2, v3, color.into()) };
                Ok(())
            }
            Shape3D::TriangleStrip(points) => {
                let ptr = points.as_ptr();
                unsafe { draw_triangle_strip_3d(ptr, points.len() as i32, color.into()) };
                Ok(())
            }
            Shape3D::Cube { position, size } => {
                unsafe { draw_cube_v(position, size, color.into()) };
                Ok(())
            }
            Shape3D::Sphere { center, radius, .. } => {
                unsafe { draw_sphere(center, radius, color.into()) };
                Ok(())
            }
            Shape3D::Cylinder {
                start_pos,
                end_pos,
                start_radius,
                end_radius,
                slices,
            } => {
                unsafe {
                    draw_cylinder_ex(
                        start_pos,
                        end_pos,
                        start_radius,
                        end_radius,
                        slices as i32,
                        color.into(),
                    )
                };
                Ok(())
            }
            Shape3D::Capsule {
                start_pos,
                end_pos,
                radius,
                slices,
                rings,
            } => {
                unsafe {
                    draw_capsule(
                        start_pos,
                        end_pos,
                        radius,
                        slices as i32,
                        rings as i32,
                        color.into(),
                    )
                };
                Ok(())
            }
            Shape3D::Plane { center, size } => {
                unsafe { draw_plane(center, size, color.into()) };
                Ok(())
            }
            Shape3D::Grid { slices, spacing } => {
                unsafe { draw_grid(slices as i32, spacing) };
                Ok(())
            }
            Shape3D::Ray(ray) => {
                unsafe { draw_ray(ray, color.into()) };
                Ok(())
            }
        }
    }

    fn draw_wires<'t>(
        &mut self,
        shape: impl Into<Shape3D<'t>>,
        color: impl Into<Color>,
    ) -> crate::error::Result<()> {
        match shape.into() {
            Shape3D::Line(start, end) => {
                unsafe { draw_line_3d(start, end, color.into()) };
                Ok(())
            }
            Shape3D::Cube { position, size } => {
                unsafe { draw_cube_wires_v(position, size, color.into()) };
                Ok(())
            }
            Shape3D::Sphere {
                center,
                radius,
                rings,
                slices,
            } => {
                unsafe {
                    draw_sphere_wires(center, radius, rings as i32, slices as i32, color.into())
                };
                Ok(())
            }
            Shape3D::Cylinder {
                start_pos,
                end_pos,
                start_radius,
                end_radius,
                slices,
            } => {
                unsafe {
                    draw_cylinder_wires_ex(
                        start_pos,
                        end_pos,
                        start_radius,
                        end_radius,
                        slices as i32,
                        color.into(),
                    )
                };
                Ok(())
            }
            Shape3D::Capsule {
                start_pos,
                end_pos,
                radius,
                slices,
                rings,
            } => {
                unsafe {
                    draw_capsule_wires(
                        start_pos,
                        end_pos,
                        radius,
                        slices as i32,
                        rings as i32,
                        color.into(),
                    )
                };
                Ok(())
            }
            Shape3D::Grid { slices, spacing } => {
                unsafe { draw_grid(slices as i32, spacing) };
                Ok(())
            }
            Shape3D::Ray(ray) => {
                unsafe { draw_ray(ray, color.into()) };
                Ok(())
            }
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

    fn draw_model(&mut self, model: &Model, position: Vector3, scale: f32, tint: impl Into<Color>) {
        unsafe { draw_model(model.as_raw(), position, scale, tint.into()) }
    }

    fn draw_model_ex(
        &mut self,
        model: &Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: impl Into<Color>,
    ) {
        unsafe {
            draw_model_ex(
                model.as_raw(),
                position,
                rotation_axis,
                rotation_angle,
                scale,
                tint.into(),
            )
        }
    }

    fn draw_model_wires(
        &mut self,
        model: &Model,
        position: Vector3,
        scale: f32,
        tint: impl Into<Color>,
    ) {
        unsafe { draw_model_wires(model.as_raw(), position, scale, tint.into()) }
    }

    fn draw_model_wires_ex(
        &mut self,
        model: &Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: impl Into<Color>,
    ) {
        unsafe {
            draw_model_wires_ex(
                model.as_raw(),
                position,
                rotation_axis,
                rotation_angle,
                scale,
                tint.into(),
            )
        }
    }

    fn draw_model_points(
        &mut self,
        model: &Model,
        position: Vector3,
        scale: f32,
        tint: impl Into<Color>,
    ) {
        unsafe { draw_model_points(model.as_raw(), position, scale, tint.into()) }
    }

    fn draw_model_points_ex(
        &mut self,
        model: &Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: impl Into<Color>,
    ) {
        unsafe {
            draw_model_points_ex(
                model.as_raw(),
                position,
                rotation_axis,
                rotation_angle,
                scale,
                tint.into(),
            )
        }
    }

    fn draw_bounding_box(&mut self, box_: BoundingBox, color: impl Into<Color>) {
        unsafe { draw_bounding_box(box_, color.into()) }
    }

    fn draw_billboard(
        &mut self,
        camera: &Camera3D,
        texture: &Texture,
        position: impl Into<Vector3>,
        scale: f32,
        tint: impl Into<Color>,
    ) {
        unsafe {
            draw_billboard(
                *camera,
                texture.as_raw(),
                position.into(),
                scale,
                tint.into(),
            )
        }
    }

    fn draw_billboard_clipped(
        &mut self,
        camera: &Camera3D,
        texture: &Texture,
        source: impl Into<Rectangle>,
        position: impl Into<Vector3>,
        size: impl Into<Vector2>,
        tint: impl Into<Color>,
    ) {
        unsafe {
            draw_billboard_rec(
                *camera,
                texture.as_raw(),
                source.into(),
                position.into(),
                size.into(),
                tint.into(),
            )
        }
    }

    fn draw_billboard_pro(
        &mut self,
        camera: &Camera3D,
        texture: &Texture,
        src: impl Into<Rectangle>,
        position: Vector3,
        up: impl Into<Vector3>,
        size: impl Into<Vector2>,
        origin: impl Into<Vector2>,
        rotation: f32,
        tint: impl Into<Color>,
    ) {
        unsafe {
            draw_billboard_pro(
                *camera,
                texture.as_raw(),
                src.into(),
                position,
                up.into(),
                size.into(),
                origin.into(),
                rotation,
                tint.into(),
            )
        }
    }

    fn draw_mesh(&mut self, mesh: &Mesh, material: &Material, transform: impl Into<Matrix>) {
        unsafe { draw_mesh(mesh.as_raw(), material.as_raw(), transform.into()) }
    }

    fn draw_mesh_instanced<M: Into<Matrix> + Copy>(
        &mut self,
        mesh: &Mesh,
        material: &Material,
        transforms: impl AsRef<[M]> + ExactSizeIterator,
    ) {
        let matrices = transforms
            .as_ref()
            .iter()
            .map(|m| (*m).into())
            .collect::<Vec<_>>();
        let ptr = matrices.as_ptr();
        unsafe {
            draw_mesh_instanced(
                mesh.as_raw(),
                material.as_raw(),
                ptr,
                transforms.len() as i32,
            )
        }
    }
}

impl Drawing<'_> {
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

impl Drawables3D for Drawing3D<'_> {}

impl Drop for Drawing3D<'_> {
    fn drop(&mut self) {
        unsafe { end_mode_3d() }
    }
}

#[allow(dead_code)]
pub struct DrawingVr<'a>(MutexGuard<'a, ()>);

impl Drawables3D for DrawingVr<'_> {}

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

impl From<Camera> for Matrix {
    fn from(val: Camera) -> Self {
        unsafe { get_camera_matrix(val) }
    }
}

impl Model {
    pub fn from_file(file_name: impl AsRef<Path>) -> Result<Self> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let model = unsafe { load_model(file_name.as_ptr()) };
        let ptr = addr_of!(model);

        if !unsafe { is_model_valid(ptr.read()) } {
            return Err(Error::UnableToLoad("model"));
        }

        Ok(Self::owned(model))
    }

    pub fn bounding_box(&self) -> BoundingBox {
        unsafe { get_model_bounding_box(self.as_raw()) }
    }

    pub fn set_mesh_material(&mut self, mesh_id: u32, material_id: u32) {
        unsafe { set_model_mesh_material(self.as_mut_ptr(), mesh_id as i32, material_id as i32) }
    }
}

impl From<Mesh> for Model {
    fn from(mesh: Mesh) -> Self {
        Self::owned(unsafe { load_model_from_mesh(mesh.as_raw()) })
    }
}

impl Mesh {
    pub fn from_polygon(sides: u32, radius: f32) -> Self {
        Self::owned(unsafe { gen_mesh_poly(sides as i32, radius) })
    }

    pub fn from_plane(width: f32, height: f32, res_x: i32, res_y: i32) -> Self {
        Self::owned(unsafe { gen_mesh_plane(width, height, res_x, res_y) })
    }

    pub fn from_planev(size: impl Into<Vector2>, res: impl Into<Vector2>) -> Self {
        let size = size.into();
        let res = res.into();
        let (res_x, res_y): (i32, i32) = res.into();
        Self::owned(unsafe { gen_mesh_plane(size.x, size.y, res_x, res_y) })
    }

    pub fn from_cube(width: f32, height: f32, length: f32) -> Self {
        Self::owned(unsafe { gen_mesh_cube(width, height, length) })
    }

    pub fn from_cubev(size: impl Into<Vector3>) -> Self {
        let size = size.into();
        Self::owned(unsafe { gen_mesh_cube(size.x, size.y, size.z) })
    }

    pub fn from_sphere(radius: f32, rings: i32, slices: i32) -> Self {
        Self::owned(unsafe { gen_mesh_sphere(radius, rings, slices) })
    }

    pub fn from_hemisphere(radius: f32, rings: i32, slices: i32) -> Self {
        Self::owned(unsafe { gen_mesh_hemi_sphere(radius, rings, slices) })
    }

    pub fn from_cylinder(radius: f32, height: f32, slices: i32) -> Self {
        Self::owned(unsafe { gen_mesh_cylinder(radius, height, slices) })
    }

    pub fn from_cone(radius: f32, height: f32, slices: i32) -> Self {
        Self::owned(unsafe { gen_mesh_cone(radius, height, slices) })
    }

    pub fn from_torus(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Self {
        Self::owned(unsafe { gen_mesh_torus(radius, size, rad_seg, sides) })
    }

    pub fn from_knot(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Self {
        Self::owned(unsafe { gen_mesh_knot(radius, size, rad_seg, sides) })
    }

    pub fn from_heightmap(heightmap: &Image, size: impl Into<Vector3>) -> Self {
        let size = size.into();
        Self::owned(unsafe { gen_mesh_heightmap(heightmap.as_raw(), size) })
    }

    pub fn from_cubic_map(cubic_map: &Image, size: impl Into<Vector3>) -> Self {
        let size = size.into();
        Self::owned(unsafe { gen_mesh_cubicmap(cubic_map.as_raw(), size) })
    }

    pub fn upload(&mut self, dynamic: bool) -> Result<()> {
        unsafe { upload_mesh(self.as_mut_ptr(), dynamic) }
        Ok(())
    }

    pub fn bounding_box(&self) -> BoundingBox {
        unsafe { get_mesh_bounding_box(self.as_raw()) }
    }

    pub fn compute_tangents(&mut self) {
        unsafe { gen_mesh_tangents(self.as_mut_ptr()) }
    }

    pub fn save(&self, file_name: impl AsRef<Path>) -> Result<()> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        unsafe {
            export_mesh(self.as_raw(), file_name.as_ptr());
        }
        Ok(())
    }
}

impl Material {
    pub fn load_materials(file_name: impl AsRef<Path>) -> Result<Vec<Self>> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let mut count: i32 = 0;
        let materials = unsafe { load_materials(file_name.as_ptr(), addr_of_mut!(count)) };
        let mut vec = Vec::with_capacity(count as usize);

        for i in 0..count {
            let ptr = unsafe { materials.add(i as usize) };
            if !unsafe { is_material_valid(ptr.read()) } {
                return Err(Error::UnableToLoad("material"));
            }
            vec.push(Self::owned(unsafe { ptr.read() }));
        }

        Ok(vec)
    }

    pub fn set_texture(&mut self, map_type: MaterialMapIndex, texture: &Texture) {
        unsafe { set_material_texture(self.as_mut_ptr(), map_type as i32, texture.as_raw()) }
    }
}

impl Default for Material {
    fn default() -> Self {
        // it's very important that we don't claim ownership of the default material,
        // otherwise the Drop impl will attempt to unload the default material causing a segfault
        Self::unowned(unsafe { load_material_default() })
    }
}

impl ModelAnimation {
    pub fn load_animations(file_name: impl AsRef<Path>, model: &Model) -> Result<Vec<Self>> {
        let file_name = file_name.as_ref().as_os_str().as_encoded_bytes();
        let file_name = CString::new(file_name)?;
        let mut count: i32 = 0;
        let animations = unsafe { load_model_animations(file_name.as_ptr(), addr_of_mut!(count)) };
        let mut vec = Vec::with_capacity(count as usize);

        for i in 0..count {
            let ptr = unsafe { animations.add(i as usize) };
            if !unsafe { is_model_animation_valid(model.as_raw(), ptr.read()) } {
                return Err(Error::UnableToLoad("model animation"));
            }
            vec.push(Self::owned(unsafe { ptr.read() }));
        }

        Ok(vec)
    }

    pub fn update(&mut self, model: &Model, frame: i32) {
        unsafe { update_model_animation(model.as_raw(), self.as_raw(), frame) }
    }

    pub fn update_bones(&mut self, model: &Model, frame: i32) {
        unsafe { update_model_animation_bones(model.as_raw(), self.as_raw(), frame) }
    }
}

impl Ray {
    pub fn raycast_sphere(&self, center: Vector3, radius: f32) -> RayCollision {
        unsafe { get_ray_collision_sphere(*self, center, radius) }
    }

    pub fn raycast_box(&self, bounding_box: impl Into<BoundingBox>) -> RayCollision {
        unsafe { get_ray_collision_box(*self, bounding_box.into()) }
    }

    pub fn raycast_mesh(&self, mesh: &Mesh, transform: impl Into<Matrix>) -> RayCollision {
        unsafe { get_ray_collision_mesh(*self, mesh.as_raw(), transform.into()) }
    }

    pub fn raycast_triangle(&self, p1: Vector3, p2: Vector3, p3: Vector3) -> RayCollision {
        unsafe { get_ray_collision_triangle(*self, p1, p2, p3) }
    }

    pub fn raycast_quad(&self, p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3) -> RayCollision {
        unsafe { get_ray_collision_quad(*self, p1, p2, p3, p4) }
    }
}
