#![allow(clippy::missing_transmute_annotations)]

use crate::graphics::Shape2D;
use crate::sys::*;
pub use crate::sys::{
    PhysicsShape, CIRCLE_VERTICES, COLLISION_ITERATIONS, MAX_BODIES, MAX_MANIFOLDS, MAX_VERTICES,
    PENETRATION_ALLOWANCE, PENETRATION_CORRECTION,
};
use crate::{guarded, info, newtype};
use std::sync::MutexGuard;

newtype!(PhysicsBody);

impl Drop for PhysicsBody {
    fn drop(&mut self) {
        eprintln!("PhysicsBody {} dropped", self.id());
        if !self.0.is_null() && self.id() < MAX_BODIES {
            unsafe { destroy_physics_body(self.as_raw()) }
        }
    }
}

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

    pub fn shape(&self) -> Option<Shape2D> {
        let verts = self.vertices();
        match verts.as_slice() {
            [] => None,

            [v] => Some(Shape2D::Pixel(*v)),
            [v1, v2] => Some(Shape2D::Line {
                start: *v1,
                end: *v2,
                thickness: None,
            }),
            [v1, v2, v3] => Some(Shape2D::Triangle {
                v1: *v3,
                v2: *v2,
                v3: *v1,
            }),
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
            [verts @ ..] => {
                let mid_x = verts.iter().map( | x | x.x ).sum::<f32>() / verts.len() as f32;
                let mid_y = verts.iter().map( | x | x.y ).sum::<f32>() / verts.len() as f32;
                let side_length = (verts[0].x - verts[1].x).hypot(verts[0].y - verts[1].y);
                let radius = side_length / (2.0 * (std::f32::consts::PI / verts.len() as f32).sin());

                Some(Shape2D::Polygon {
                    center: Vector2 { x: mid_x, y: mid_y },
                    sides: verts.len() as i32,
                    radius,
                    rotation: 0.0,
                })
            }
        }
    }

    pub fn vertices_count(&self) -> Option<usize> {
        let idx = unsafe { (*self.as_raw()).id as i32 };
        Some(unsafe { get_physics_shape_vertices_count(idx) as usize })
    }

    pub fn vertex(&self, index: usize) -> Option<Vector2> {
        let count = self.vertices_count()?;

        if index >= count {
            return None;
        }

        Some(unsafe { get_physics_shape_vertex(self.as_raw(), index as i32) })
    }

    pub fn vertices(&self) -> Vec<Vector2> {
        if let Some(count) = self.vertices_count() {
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

    pub fn id(&self) -> u32 {
        unsafe { (*self.as_raw()).id }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { (*self.as_raw()).enabled }
    }

    pub fn enabled(&mut self, enabled: bool) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).enabled = enabled;
        }
    }

    pub fn position(&self) -> Vector2 {
        unsafe { (*self.as_raw()).position }
    }

    pub fn set_position(&mut self, position: impl Into<Vector2>) {
        let position = position.into();
        unsafe {
            let ptr = self.as_raw();
            (*ptr).position = position;
        }
    }

    pub fn velocity(&self) -> Vector2 {
        unsafe { (*self.as_raw()).velocity }
    }

    pub fn set_velocity(&mut self, velocity: impl Into<Vector2>) {
        let velocity = velocity.into();
        unsafe {
            let ptr = self.as_raw();
            (*ptr).velocity = velocity;
        }
    }

    pub fn force(&self) -> Vector2 {
        unsafe { (*self.as_raw()).force }
    }

    pub fn set_force(&mut self, force: impl Into<Vector2>) {
        let force = force.into();
        unsafe {
            let ptr = self.as_raw();
            (*ptr).force = force;
        }
    }

    pub fn angular_velocity(&self) -> f32 {
        unsafe { (*self.as_raw()).angular_velocity }
    }

    pub fn set_angular_velocity(&mut self, angular_velocity: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).angular_velocity = angular_velocity;
        }
    }

    pub fn torque(&self) -> f32 {
        unsafe { (*self.as_raw()).torque }
    }

    pub fn set_torque(&mut self, torque: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).torque = torque;
        }
    }

    pub fn rotation(&self) -> f32 {
        unsafe { (*self.as_raw()).orient }
    }

    pub fn static_friction(&self) -> f32 {
        unsafe { (*self.as_raw()).static_friction }
    }

    pub fn set_static_friction(&mut self, static_friction: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).static_friction = static_friction;
        }
    }

    pub fn dynamic_friction(&self) -> f32 {
        unsafe { (*self.as_raw()).dynamic_friction }
    }

    pub fn set_dynamic_friction(&mut self, dynamic_friction: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).dynamic_friction = dynamic_friction;
        }
    }

    pub fn restitution(&self) -> f32 {
        unsafe { (*self.as_raw()).restitution }
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).restitution = restitution;
        }
    }

    pub fn gravity(&self) -> bool {
        unsafe { (*self.as_raw()).use_gravity }
    }

    pub fn use_gravity(&mut self, use_gravity: bool) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).use_gravity = use_gravity;
        }
    }

    pub fn is_grounded(&self) -> bool {
        unsafe { (*self.as_raw()).is_grounded }
    }

    pub fn grounded(&mut self, is_grounded: bool) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).is_grounded = is_grounded;
        }
    }

    pub fn allows_rotation(&self) -> bool {
        unsafe { (*self.as_raw()).freeze_orient }
    }

    pub fn allow_rotation(&mut self, freeze_orient: bool) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).freeze_orient = freeze_orient;
        }
    }

    pub fn mass(&self) -> f32 {
        unsafe { (*self.as_raw()).mass }
    }

    pub fn set_mass(&mut self, mass: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).mass = mass;
        }
    }

    pub fn inertia(&self) -> f32 {
        unsafe { (*self.as_raw()).inertia }
    }

    pub fn set_inertia(&mut self, inertia: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).inertia = inertia;
        }
    }

    pub fn inverse_mass(&self) -> f32 {
        unsafe { (*self.as_raw()).inverse_mass }
    }

pub fn set_inverse_mass(&mut self, inverse_mass: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).inverse_mass = inverse_mass;
        }
    }

    pub fn inverse_inertia(&self) -> f32 {
        unsafe { (*self.as_raw()).inverse_inertia }
    }

pub fn set_inverse_inertia(&mut self, inverse_inertia: f32) {
        unsafe {
            let ptr = self.as_raw();
            (*ptr).inverse_inertia = inverse_inertia;
        }
    }

    pub fn count() -> usize {
        unsafe { get_physics_bodies_count() as usize }
    }

    pub fn all() -> Vec<PhysicsBody> {
        let count = Self::count();
        (0..count)
            .map(|x| unsafe { get_physics_body(x as i32) })
            .map(From::from)
            .collect()
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
            Ok(unsafe { create_physics_body_circle(position, radius, density) }.into())
        }
    }

    pub fn create_rectangle(&mut self, rc: impl Into<Rectangle>, density: f32) -> crate::Result<PhysicsBody> {
        if PhysicsBody::count() >= MAX_BODIES as usize {
            Err(crate::Error::TooManyPhysicsBodies)
        } else {
            let rc = rc.into();
            let (width, height) = rc.size().into();
            Ok(unsafe { create_physics_body_rectangle(rc.position(), width, height, density) }.into())
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
            Ok(unsafe { create_physics_body_polygon(pos.into(), radius, sides as i32, density) }.into())
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
