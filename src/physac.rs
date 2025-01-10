#![allow(clippy::missing_transmute_annotations)]

use crate::sys::*;
use crate::{guarded, newtype};
use std::sync::MutexGuard;

pub use crate::sys::{
    PhysicsShape, CIRCLE_VERTICES, COLLISION_ITERATIONS, MAX_BODIES, MAX_MANIFOLDS, MAX_VERTICES,
    PENETRATION_ALLOWANCE, PENETRATION_CORRECTION,
};

newtype!(PhysicsBody, destroy_physics_body);

impl PhysicsBody {
    pub fn add_force(&mut self, force: impl Into<Vector2>) {
        unsafe {
            physics_add_force(self.0, force.into());
        }
    }

    pub fn add_torque(&mut self, torque: f32) {
        unsafe {
            physics_add_torque(self.0, torque);
        }
    }

    pub fn shatter(&mut self, position: impl Into<Vector2>, force: f32) {
        unsafe {
            physics_shatter(self.0, position.into(), force);
        }
    }

    pub fn shape(&self) -> Option<PhysicsShapeType> {
        let idx = unsafe { (*self.as_raw()).id as i32 };
        let shape = unsafe { get_physics_shape_type(idx) };

        Some(unsafe { std::mem::transmute(shape) })
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
            (0..count)
                .filter_map(|x| self.vertex(x))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn set_rotation(&mut self, radians: f32) {
        unsafe {
            set_physics_body_rotation(self.0, radians);
        }
    }

    pub fn count() -> usize {
        unsafe { get_physics_bodies_count() as usize }
    }

    pub fn all() -> Vec<PhysicsBody> {
        let count = Self::count();
        (0..count)
            .map(|x| unsafe { get_physics_body(x as i32) })
            .map(PhysicsBody)
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
    ) -> PhysicsBody {
        let position = position.into();
        PhysicsBody(unsafe { create_physics_body_circle(position, radius, density) })
    }

    pub fn create_rectangle(&mut self, rc: impl Into<Rectangle>, density: f32) -> PhysicsBody {
        let rc = rc.into();
        let (width, height) = rc.size().into();

        PhysicsBody(unsafe { create_physics_body_rectangle(rc.position(), width, height, density) })
    }

    pub fn create_polygon(
        &mut self,
        pos: impl Into<Vector2>,
        radius: f32,
        sides: u32,
        density: f32,
    ) -> PhysicsBody {
        PhysicsBody(unsafe {
            create_physics_body_polygon(pos.into(), radius, sides as i32, density)
        })
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
