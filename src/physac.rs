#![allow(clippy::missing_transmute_annotations)]

use crate::graphics::Shape2D;
use crate::sys::*;
pub use crate::sys::{
    CIRCLE_VERTICES, COLLISION_ITERATIONS, MAX_BODIES, MAX_MANIFOLDS, MAX_VERTICES,
    PENETRATION_ALLOWANCE, PENETRATION_CORRECTION,
};
use crate::{getter, guarded, newtype, property};
use std::ops::*;
use std::sync::MutexGuard;

impl Mat2 {
    pub const IDENTITY: Self = Self {
        m00: 1.0,
        m01: 0.0,
        m10: 0.0,
        m11: 1.0,
    };

    pub fn addf(&mut self, rhs: f32) {
        self.m00 += rhs;
        self.m01 += rhs;
        self.m10 += rhs;
        self.m11 += rhs;
    }

    pub fn subf(&mut self, rhs: f32) {
        self.m00 -= rhs;
        self.m01 -= rhs;
        self.m10 -= rhs;
        self.m11 -= rhs;
    }

    pub fn mulf(&mut self, rhs: f32) {
        self.m00 *= rhs;
        self.m01 *= rhs;
        self.m10 *= rhs;
        self.m11 *= rhs;
    }
}

impl<T: Into<Vector4>> From<T> for Mat2 {
    fn from(v: T) -> Self {
        let v = v.into();
        Mat2 {
            m00: v.x,
            m01: v.y,
            m10: v.z,
            m11: v.w,
        }
    }
}

impl Add for Mat2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Mat2 {
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
        }
    }
}

impl AddAssign for Mat2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Mat2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Mat2 {
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
        }
    }
}

impl SubAssign for Mat2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Mat2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Mat2 {
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11,
        }
    }
}

impl MulAssign for Mat2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

newtype!(PolygonData);

impl PolygonData {
    getter!(vertex_count: u32);

    pub fn positions(&self) -> [Vector2; 24] {
        let data = unsafe { self.as_raw() };
        data.positions
    }

    pub fn normals(&self) -> [Vector2; 24] {
        let data = unsafe { self.as_raw() };
        data.normals
    }
}

newtype!(PhysicsShape);

impl PhysicsShape {
    getter!(r#type[shape_type]: u32);
    property!(radius: f32);
    property!(transform: Mat2);

    pub fn body(&self) -> PhysicsBody {
        let ptr = unsafe { self.as_ptr() };
        PhysicsBody::unowned(unsafe { (*ptr).body })
    }

    pub fn vertices(&self) -> PolygonData {
        let ptr = unsafe { self.as_ptr() };
        let data = unsafe { (*ptr).vertex_data };
        PolygonData::unowned(data)
    }
}

newtype!(PhysicsBody, @destroy_physics_body);

impl PhysicsBody {
    pub fn add_force(&mut self, force: impl Into<Vector2>) {
        unsafe {
            physics_add_force(self.as_raw(), force.into());
        }
    }

    pub fn add_torque(&mut self, torque: f32) {
        unsafe {
            physics_add_torque(self.as_raw(), torque);
        }
    }

    pub fn shatter(&mut self, position: impl Into<Vector2>, force: f32) {
        unsafe {
            physics_shatter(self.as_raw(), position.into(), force);
        }
    }

    pub fn shape_type(&self) -> Option<PhysicsShapeType> {
        let idx = unsafe { (*self.as_raw()).id as i32 };
        let shape = unsafe { get_physics_shape_type(idx) };

        Some(unsafe { std::mem::transmute(shape) })
    }

    pub fn get_shape2d(&self) -> Option<Shape2D> {
        let verts = self.vertices();
        match verts.as_slice() {
            [] => None,

            [v] => Some(Shape2D::Pixel(*v)),
            [v1, v2] => Some(Shape2D::Line {
                start: *v1,
                end: *v2,
                thickness: None,
            }),
            [v1, v2, v3] => {
                let mut top_vert = *v3;
                let mut bottom_right_vert = *v2;
                let mut bottom_left_vert = *v1;
                let height = top_vert.y - bottom_right_vert.y;

                // because the verticies are relative to a center point, we need to shift the Y
                // coordinate of all 3 verticies down by half the triangle's height
                // to align the shape with the physics body
                top_vert.y = top_vert.y - height / 2.0;
                bottom_left_vert.y = bottom_left_vert.y - height / 2.0;
                bottom_right_vert.y = bottom_right_vert.y - height / 2.0;

                Some(Shape2D::Triangle {
                    v1: top_vert,
                    v2: bottom_right_vert,
                    v3: bottom_left_vert,
                })
            }
            [v1, v2, v3, v4] => {
                let width = v2.x - v3.x;
                let height = v3.y - v4.y;

                Some(Shape2D::Rectangle {
                    rect: Rectangle {
                        x: v1.x - width / 2.0,
                        y: v2.y - height / 2.0,
                        width,
                        height,
                    },
                    rotation: None,
                })
            }
            verts => {
                let mid_x = verts.iter().map(|x| x.x).sum::<f32>() / verts.len() as f32;
                let mid_y = verts.iter().map(|x| x.y).sum::<f32>() / verts.len() as f32;
                let side_length = (verts[0].x - verts[1].x).hypot(verts[0].y - verts[1].y);
                let radius =
                    side_length / (2.0 * (std::f32::consts::PI / verts.len() as f32).sin());

                Some(Shape2D::Polygon {
                    center: Vector2 { x: mid_x, y: mid_y + radius / 2.0 },
                    sides: verts.len() as i32,
                    radius,
                    rotation: self.rotation(),
                })
            }
        }
    }

    pub fn vertex_count(&self) -> Option<usize> {
        let idx = unsafe { (*self.as_raw()).id as i32 };
        Some(unsafe { get_physics_shape_vertices_count(idx) as usize })
    }

    pub fn vertex(&self, index: usize) -> Option<Vector2> {
        let count = self.vertex_count()?;

        if index >= count {
            return None;
        }

        Some(unsafe { get_physics_shape_vertex(self.as_raw(), index as i32) })
    }

    pub fn vertices(&self) -> Vec<Vector2> {
        if let Some(count) = self.vertex_count() {
            (0..count).filter_map(|x| self.vertex(x)).collect()
        } else {
            Vec::new()
        }
    }

    pub fn set_rotation(&mut self, radians: f32) {
        unsafe {
            set_physics_body_rotation(self.as_raw(), radians);
        }
    }

    getter!(*id: u32);
    property!(*enabled: bool);
    getter!(*position: Vector2);
    getter!(*velocity: Vector2);
    getter!(*force: Vector2);
    getter!(*angular_velocity: f32);
    getter!(*torque: f32);
    getter!(*orient[rotation]: f32);
    getter!(*inertia: f32);
    getter!(*inverse_inertia: f32);
    getter!(*mass: f32);
    getter!(*inverse_mass: f32);
    getter!(*static_friction: f32);
    getter!(*dynamic_friction: f32);
    getter!(*restitution: f32);
    property!(*use_gravity: bool);
    property!(*is_grounded: bool);
    property!(*freeze_orient[lock_rotation]: bool);

    pub fn physics_shape(&self) -> PhysicsShape {
        let shape = unsafe { self.as_ptr() };
        let shape = unsafe { shape.read() };
        let shape = unsafe { &(*shape).shape } as *const crate::sys::PhysicsShape;
        let shape = unsafe { shape.read() };
        PhysicsShape::unowned(shape)
    }

    pub fn count() -> usize {
        unsafe { get_physics_bodies_count() as usize }
    }

    pub fn all() -> Vec<PhysicsBody> {
        let count = Self::count();
        (0..count)
            .map(|x| unsafe { get_physics_body(x as i32) })
            .map(PhysicsBody::unowned)
            .collect()
    }
}

newtype!(PhysicsManifold);

impl PhysicsManifold {
    getter!(*id: u32);
    getter!(*penetration: f32);
    getter!(*normal: Vector2);
    getter!(*contacts_count: u32);
    getter!(*restitution: f32);
    getter!(*dynamic_friction: f32);
    getter!(*static_friction: f32);

    pub fn contacts(&self) -> &[Vector2] {
        let data = unsafe { self.as_raw() };
        unsafe { (*data).contacts.as_slice() }
    }

    pub fn body_a(&self) -> PhysicsBody {
        let data = unsafe { self.as_raw() };
        let body = unsafe { (*data).body_a };
        PhysicsBody::unowned(body)
    }

    pub fn body_b(&self) -> PhysicsBody {
        let data = unsafe { self.as_raw() };
        let body = unsafe { (*data).body_b };
        PhysicsBody::unowned(body)
    }
}

guarded!(Physics);

impl<'a> Physics<'a> {
    pub(crate) fn get(guard: MutexGuard<'a, ()>) -> Self {
        if !unsafe { is_physics_enabled() } {
            unsafe {
                init_physics();
            }
        }
        Self::new(guard)
    }

    pub fn set_gravity(&mut self, x: f32, y: f32) {
        unsafe {
            set_physics_gravity(x, y);
        }
    }

    pub fn set_gravityv(&mut self, gravity: impl Into<Vector2>) {
        let gravity = gravity.into();
        unsafe {
            set_physics_gravity(gravity.x, gravity.y);
        }
    }

    pub fn create_circle(
        &mut self,
        position: impl Into<Vector2>,
        radius: f32,
        density: f32,
    ) -> crate::Result<PhysicsBody> {
        if PhysicsBody::count() >= MAX_BODIES as usize {
            Err(crate::Error::TooManyPhysicsBodies)
        } else {
            let position = position.into();
            Ok(PhysicsBody::owned(unsafe {
                create_physics_body_circle(position, radius, density)
            }))
        }
    }

    pub fn create_rectangle(
        &mut self,
        rc: impl Into<Rectangle>,
        density: f32,
    ) -> crate::Result<PhysicsBody> {
        if PhysicsBody::count() >= MAX_BODIES as usize {
            Err(crate::Error::TooManyPhysicsBodies)
        } else {
            let rc = rc.into();
            let (width, height) = rc.size().into();
            Ok(PhysicsBody::owned(unsafe {
                create_physics_body_rectangle(rc.position(), width, height, density)
            }))
        }
    }

    pub fn create_polygon(
        &mut self,
        pos: impl Into<Vector2>,
        radius: f32,
        sides: u32,
        density: f32,
    ) -> crate::Result<PhysicsBody> {
        if PhysicsBody::count() >= MAX_BODIES as usize {
            Err(crate::Error::TooManyPhysicsBodies)
        } else {
            Ok(PhysicsBody::owned(unsafe {
                create_physics_body_polygon(pos.into(), radius, sides as i32, density)
            }))
        }
    }
}

impl Drop for Physics<'_> {
    fn drop(&mut self) {
        if unsafe { is_physics_enabled() } {
            unsafe {
                close_physics();
            }
        }
    }
}
