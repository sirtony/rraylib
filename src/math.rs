use crate::sys::*;
pub use crate::sys::{Matrix, Rectangle, Vector2, Vector3, Vector4};
use std::ops::*;
use std::ptr::addr_of_mut;

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn top_left(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn top_right(&self) -> Vector2 {
        Vector2::new(self.x + self.width, self.y)
    }

    pub fn bottom_left(&self) -> Vector2 {
        Vector2::new(self.x, self.y + self.height)
    }

    pub fn position(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn size(&self) -> Vector2 {
        Vector2::new(self.width, self.height)
    }

    pub fn bottom_right(&self) -> Vector2 {
        Vector2::new(self.x + self.width, self.y + self.height)
    }

    pub fn growf(&mut self, amount: f32) {
        self.x -= amount;
        self.y -= amount;
        self.width += amount * 2.0;
        self.height += amount * 2.0;
    }

    pub fn grow(&mut self, amount: Vector2) {
        self.x -= amount.x;
        self.y -= amount.y;
        self.width += amount.x * 2.0;
        self.height += amount.y * 2.0;
    }

    pub fn shrinkf(&mut self, amount: f32) {
        self.x += amount;
        self.y += amount;
        self.width -= amount * 2.0;
        self.height -= amount * 2.0;
    }

    pub fn shrink(&mut self, amount: Vector2) {
        self.x += amount.x;
        self.y += amount.y;
        self.width -= amount.x * 2.0;
        self.height -= amount.y * 2.0;
    }

    pub fn move_by(&mut self, amount: Vector2) {
        self.x += amount.x;
        self.y += amount.y;
    }

    pub fn move_byf(&mut self, amount: f32) {
        self.x += amount;
        self.y += amount;
    }

    pub fn move_to(&mut self, position: Vector2) {
        self.x = position.x;
        self.y = position.y;
    }

    pub fn move_tof(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn contains_point(&self, position: &Vector2) -> bool {
        unsafe { check_collision_point_rec(*position, *self) }
    }

    pub fn contains_pointf(&self, x: f32, y: f32) -> bool {
        unsafe { check_collision_point_rec(Vector2::new(x, y), *self) }
    }

    pub fn contains_rectangle(&self, other: &Rectangle) -> bool {
        self.contains_point(&other.top_left())
            && self.contains_point(&other.top_right())
            && self.contains_point(&other.bottom_left())
            && self.contains_point(&other.bottom_right())
    }

    pub fn collides_with(&self, other: &Rectangle) -> bool {
        unsafe { check_collision_recs(*self, *other) }
    }

    pub fn collision_area(&self, other: &Rectangle) -> Rectangle {
        unsafe { get_collision_rec(*self, *other) }
    }

    pub fn center(&self) -> (i32, i32) {
        (
            self.x as i32 + self.width as i32 / 2,
            self.y as i32 + self.height as i32 / 2,
        )
    }

    pub fn centerv(&self) -> Vector2 {
        Vector2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }
}

impl Eq for Rectangle {}
impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        let (pos1, size1): (Vector2, Vector2) = (*self).into();
        let (pos2, size2): (Vector2, Vector2) = (*other).into();

        pos1 == pos2 && size1 == size2
    }
}

impl From<(Vector2, Vector2)> for Rectangle {
    fn from((position, size): (Vector2, Vector2)) -> Self {
        Self {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        }
    }
}

impl From<Rectangle> for (Vector2, Vector2) {
    fn from(val: Rectangle) -> Self {
        (
            Vector2::new(val.x, val.y),
            Vector2::new(val.width, val.height),
        )
    }
}

impl From<[Vector2; 2]> for Rectangle {
    fn from([position, size]: [Vector2; 2]) -> Self {
        Self {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        }
    }
}

impl From<Rectangle> for [Vector2; 2] {
    fn from(val: Rectangle) -> Self {
        [
            Vector2::new(val.x, val.y),
            Vector2::new(val.width, val.height),
        ]
    }
}

impl From<(f32, f32, f32, f32)> for Rectangle {
    fn from((x, y, width, height): (f32, f32, f32, f32)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Rectangle> for (f32, f32, f32, f32) {
    fn from(val: Rectangle) -> Self {
        (val.x, val.y, val.width, val.height)
    }
}

impl From<[f32; 4]> for Rectangle {
    fn from([x, y, width, height]: [f32; 4]) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Rectangle> for [f32; 4] {
    fn from(val: Rectangle) -> Self {
        [val.x, val.y, val.width, val.height]
    }
}

impl From<(i32, i32, i32, i32)> for Rectangle {
    fn from((x, y, width, height): (i32, i32, i32, i32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        }
    }
}

impl From<Rectangle> for (i32, i32, i32, i32) {
    fn from(val: Rectangle) -> Self {
        (
            val.x as i32,
            val.y as i32,
            val.width as i32,
            val.height as i32,
        )
    }
}

impl From<[i32; 4]> for Rectangle {
    fn from([x, y, width, height]: [i32; 4]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        }
    }
}

impl From<Rectangle> for [i32; 4] {
    fn from(val: Rectangle) -> Self {
        [
            val.x as i32,
            val.y as i32,
            val.width as i32,
            val.height as i32,
        ]
    }
}

impl From<(u32, u32, u32, u32)> for Rectangle {
    fn from((x, y, width, height): (u32, u32, u32, u32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        }
    }
}

impl From<Rectangle> for (u32, u32, u32, u32) {
    fn from(val: Rectangle) -> Self {
        (
            val.x as u32,
            val.y as u32,
            val.width as u32,
            val.height as u32,
        )
    }
}

impl From<[u32; 4]> for Rectangle {
    fn from([x, y, width, height]: [u32; 4]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
        }
    }
}

impl From<Rectangle> for [u32; 4] {
    fn from(val: Rectangle) -> Self {
        [
            val.x as u32,
            val.y as u32,
            val.width as u32,
            val.height as u32,
        ]
    }
}

impl From<Vector4> for Rectangle {
    fn from(v: Vector4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            width: v.z,
            height: v.w,
        }
    }
}

impl From<Rectangle> for Vector4 {
    fn from(val: Rectangle) -> Self {
        Vector4::new(val.x, val.y, val.width, val.height)
    }
}

impl Vector2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn addf(&mut self, value: f32) {
        *self = unsafe { vector2_add_value(*self, value) }
    }

    pub fn subf(&mut self, value: f32) {
        *self = unsafe { vector2_subtract_value(*self, value) }
    }

    pub fn scale(&mut self, value: f32) {
        *self = unsafe { vector2_scale(*self, value) }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn len(&self) -> f32 {
        unsafe { vector2_length(*self) }
    }

    pub fn len_squared(&self) -> f32 {
        unsafe { vector2_length_sqr(*self) }
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        unsafe { vector2_dot_product(*self, *other) }
    }

    pub fn distance(&self, other: &Vector2) -> f32 {
        unsafe { vector2_distance(*self, *other) }
    }

    pub fn distance_squared(&self, other: &Vector2) -> f32 {
        unsafe { vector2_distance_sqr(*self, *other) }
    }

    pub fn angle(&self, other: &Vector2) -> f32 {
        unsafe { vector2_angle(*self, *other) }
    }

    pub fn normalize(&self) -> Self {
        unsafe { vector2_normalize(*self) }
    }

    pub fn transform(&self, mat: &Matrix) -> Self {
        unsafe { vector2_transform(*self, *mat) }
    }

    pub fn lerp(&self, target: &Vector2, amount: f32) -> Self {
        unsafe { vector2_lerp(*self, *target, amount) }
    }

    pub fn reflect(&self, normal: &Vector2) -> Self {
        unsafe { vector2_reflect(*self, *normal) }
    }

    pub fn rotate(&self, degs: f32) -> Self {
        unsafe { vector2_rotate(*self, degs) }
    }

    pub fn move_towards(&self, target: &Vector2, max_distance: f32) -> Self {
        unsafe { vector2_move_towards(*self, *target, max_distance) }
    }

    pub fn invert(&self) -> Self {
        unsafe { vector2_invert(*self) }
    }

    pub fn clamp(&self, min: &Vector2, max: &Vector2) -> Self {
        unsafe { vector2_clamp(*self, *min, *max) }
    }

    pub fn clampf(&self, min: f32, max: f32) -> Self {
        unsafe { vector2_clamp_value(*self, min, max) }
    }
}

impl Eq for Vector2 {}
impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { vector2_equals(*self, *other) == 1 }
    }
}

impl IntoIterator for Vector2 {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y].into_iter()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unsafe { vector2_add(self, rhs) }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unsafe { vector2_subtract(self, rhs) }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe { vector2_multiply(self, rhs) }
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unsafe { vector2_divide(self, rhs) }
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        unsafe { vector2_negate(self) }
    }
}

impl From<Vector2> for [f32; 2] {
    fn from(val: Vector2) -> Self {
        [val.x, val.y]
    }
}

impl From<[f32; 2]> for Vector2 {
    fn from([x, y]: [f32; 2]) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<Vector2> for (f32, f32) {
    fn from(val: Vector2) -> Self {
        (val.x, val.y)
    }
}

impl From<(i32, i32)> for Vector2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<Vector2> for (i32, i32) {
    fn from(val: Vector2) -> Self {
        (val.x as i32, val.y as i32)
    }
}

impl From<[i32; 2]> for Vector2 {
    fn from([x, y]: [i32; 2]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<Vector2> for [i32; 2] {
    fn from(val: Vector2) -> Self {
        [val.x as i32, val.y as i32]
    }
}

impl From<(u32, u32)> for Vector2 {
    fn from((x, y): (u32, u32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<Vector2> for (u32, u32) {
    fn from(val: Vector2) -> Self {
        (val.x as u32, val.y as u32)
    }
}

impl From<[u32; 2]> for Vector2 {
    fn from([x, y]: [u32; 2]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
}

impl From<Vector2> for [u32; 2] {
    fn from(val: Vector2) -> Self {
        [val.x as u32, val.y as u32]
    }
}

impl Vector3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };

    pub const DOWN: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };

    pub const RIGHT: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };

    pub const LEFT: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };

    pub const FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    pub const BACK: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn addf(&mut self, value: f32) {
        *self = unsafe { vector3_add_value(*self, value) }
    }

    pub fn subf(&mut self, value: f32) {
        *self = unsafe { vector3_subtract_value(*self, value) }
    }

    pub fn scale(&mut self, value: f32) {
        *self = unsafe { vector3_scale(*self, value) }
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        unsafe { vector3_cross_product(*self, *other) }
    }

    pub fn perp(&self) -> Self {
        unsafe { vector3_perpendicular(*self) }
    }

    pub fn len(&self) -> f32 {
        unsafe { vector3_length(*self) }
    }

    pub fn len_squared(&self) -> f32 {
        unsafe { vector3_length_sqr(*self) }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        unsafe { vector3_dot_product(*self, *other) }
    }

    pub fn distance(&self, other: &Vector3) -> f32 {
        unsafe { vector3_distance(*self, *other) }
    }

    pub fn distance_squared(&self, other: &Vector3) -> f32 {
        unsafe { vector3_distance_sqr(*self, *other) }
    }

    pub fn angle(&self, other: &Vector3) -> f32 {
        unsafe { vector3_angle(*self, *other) }
    }

    pub fn normalize(&self) -> Self {
        unsafe { vector3_normalize(*self) }
    }

    pub fn transform(&self, mat: &Matrix) -> Self {
        unsafe { vector3_transform(*self, *mat) }
    }

    pub fn quat_rotate(&self, q: &Quaternion) -> Self {
        unsafe { vector3_rotate_by_quaternion(*self, *q) }
    }

    pub fn rotate(&self, axis: &Vector3, degs: f32) -> Self {
        unsafe { vector3_rotate_by_axis_angle(*self, *axis, degs) }
    }

    pub fn lerp(&self, target: &Vector3, amount: f32) -> Self {
        unsafe { vector3_lerp(*self, *target, amount) }
    }

    pub fn reflect(&self, normal: &Vector3) -> Self {
        unsafe { vector3_reflect(*self, *normal) }
    }

    pub fn min(&self, other: &Vector3) -> Self {
        unsafe { vector3_min(*self, *other) }
    }

    pub fn max(&self, other: &Vector3) -> Self {
        unsafe { vector3_max(*self, *other) }
    }

    pub fn barycenter(&self, i: &Vector3, j: &Vector3, k: &Vector3) -> Self {
        unsafe { vector3_barycenter(*self, *i, *j, *k) }
    }

    pub fn unproject(&self, proj: &Matrix, view: &Matrix) -> Self {
        unsafe { vector3_unproject(*self, *proj, *view) }
    }

    pub fn invert(&self) -> Self {
        unsafe { vector3_invert(*self) }
    }

    pub fn clamp(&self, min: &Vector3, max: &Vector3) -> Self {
        unsafe { vector3_clamp(*self, *min, *max) }
    }

    pub fn clampf(&self, min: f32, max: f32) -> Self {
        unsafe { vector3_clamp_value(*self, min, max) }
    }

    pub fn refract(&self, normal: &Vector3, ior: f32) -> Self {
        unsafe { vector3_refract(*self, *normal, ior) }
    }
}

impl Eq for Vector3 {}
impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { vector3_equals(*self, *other) == 1 }
    }
}

impl IntoIterator for Vector3 {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z].into_iter()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unsafe { vector3_add(self, rhs) }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unsafe { vector3_subtract(self, rhs) }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe { vector3_multiply(self, rhs) }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Vector3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unsafe { vector3_divide(self, rhs) }
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        unsafe { vector3_negate(self) }
    }
}

impl From<Vector3> for [f32; 3] {
    fn from(val: Vector3) -> Self {
        [val.x, val.y, val.z]
    }
}

impl From<[f32; 3]> for Vector3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vector3> for (f32, f32, f32) {
    fn from(val: Vector3) -> Self {
        (val.x, val.y, val.z)
    }
}

impl From<(i32, i32, i32)> for Vector3 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }
}

impl From<Vector3> for (i32, i32, i32) {
    fn from(val: Vector3) -> Self {
        (val.x as i32, val.y as i32, val.z as i32)
    }
}

impl From<Vector3> for [i32; 3] {
    fn from(val: Vector3) -> Self {
        [val.x as i32, val.y as i32, val.z as i32]
    }
}

impl From<[i32; 3]> for Vector3 {
    fn from([x, y, z]: [i32; 3]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }
}

impl From<(u32, u32, u32)> for Vector3 {
    fn from((x, y, z): (u32, u32, u32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }
}

impl From<Vector3> for (u32, u32, u32) {
    fn from(val: Vector3) -> Self {
        (val.x as u32, val.y as u32, val.z as u32)
    }
}

impl From<Vector3> for [u32; 3] {
    fn from(val: Vector3) -> Self {
        [val.x as u32, val.y as u32, val.z as u32]
    }
}

impl From<[u32; 3]> for Vector3 {
    fn from([x, y, z]: [u32; 3]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }
}

impl Vector4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_axis_angle(axis: &Vector3, angle: f32) -> Self {
        unsafe { quaternion_from_axis_angle(*axis, angle) }
    }

    pub fn cubic_hermite_spline(
        q1: impl Into<Quaternion>,
        out_tangent: impl Into<Quaternion>,
        q2: impl Into<Quaternion>,
        in_tangent: impl Into<Quaternion>,
        t: f32,
    ) -> Self {
        unsafe {
            quaternion_cubic_hermite_spline(
                q1.into(),
                out_tangent.into(),
                q2.into(),
                in_tangent.into(),
                t,
            )
        }
    }

    pub fn identity() -> Self {
        unsafe { quaternion_identity() }
    }

    pub fn addf(&mut self, value: f32) {
        *self = unsafe { vector4_add_value(*self, value) }
    }

    pub fn subf(&mut self, value: f32) {
        *self = unsafe { vector4_subtract_value(*self, value) }
    }

    pub fn scale(&mut self, value: f32) {
        *self = unsafe { vector4_scale(*self, value) }
    }

    pub fn len(&self) -> f32 {
        unsafe { vector4_length(*self) }
    }

    pub fn len_squared(&self) -> f32 {
        unsafe { vector4_length_sqr(*self) }
    }

    pub fn normalize(&self) -> Self {
        unsafe { vector4_normalize(*self) }
    }

    pub fn dot(&self, other: &Vector4) -> f32 {
        unsafe { vector4_dot_product(*self, *other) }
    }

    pub fn lerp(&self, target: &Vector4, amount: f32) -> Self {
        unsafe { vector4_lerp(*self, *target, amount) }
    }

    pub fn nlerp(&self, target: &Vector4, amount: f32) -> Self {
        unsafe { quaternion_nlerp(*self, *target, amount) }
    }

    pub fn slerp(&self, target: &Vector4, amount: f32) -> Self {
        unsafe { quaternion_slerp(*self, *target, amount) }
    }

    pub fn invert(&self) -> Self {
        unsafe { vector4_invert(*self) }
    }

    pub fn move_towards(&self, target: &Vector4, max_distance: f32) -> Self {
        unsafe { vector4_move_towards(*self, *target, max_distance) }
    }

    pub fn distance(&self, other: &Vector4) -> f32 {
        unsafe { vector4_distance(*self, *other) }
    }

    pub fn min(&self, other: &Vector4) -> Self {
        unsafe { vector4_min(*self, *other) }
    }

    pub fn max(&self, other: &Vector4) -> Self {
        unsafe { vector4_max(*self, *other) }
    }

    pub fn transform(&self, mat: &Matrix) -> Self {
        unsafe { quaternion_transform(*self, *mat) }
    }
}

impl Eq for Vector4 {}
impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { vector4_equals(*self, *other) == 1 }
    }
}

impl IntoIterator for Vector4 {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z, self.w].into_iter()
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unsafe { vector4_add(self, rhs) }
    }
}

impl AddAssign for Vector4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unsafe { vector4_subtract(self, rhs) }
    }
}

impl SubAssign for Vector4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vector4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe { vector4_multiply(self, rhs) }
    }
}

impl MulAssign for Vector4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Vector4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unsafe { vector4_divide(self, rhs) }
    }
}

impl DivAssign for Vector4 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Vector4 {
    type Output = Self;

    fn neg(self) -> Self {
        unsafe { vector4_negate(self) }
    }
}

impl From<Vector4> for Vector3 {
    fn from(val: Vector4) -> Self {
        unsafe { quaternion_to_euler(val) }
    }
}

impl From<(Vector3, Vector3)> for Vector4 {
    fn from((from, to): (Vector3, Vector3)) -> Self {
        unsafe { quaternion_from_vector3_to_vector3(from, to) }
    }
}

impl From<[Vector3; 2]> for Vector4 {
    fn from([from, to]: [Vector3; 2]) -> Self {
        unsafe { quaternion_from_vector3_to_vector3(from, to) }
    }
}

impl From<Vector4> for Matrix {
    fn from(val: Vector4) -> Self {
        unsafe { quaternion_to_matrix(val) }
    }
}

impl From<Vector4> for (Vector3, f32) {
    fn from(val: Vector4) -> Self {
        let mut axis = Vector3::ZERO;
        let mut angle = 0.0;
        unsafe { quaternion_to_axis_angle(val, addr_of_mut!(axis), addr_of_mut!(angle)) }

        (axis, angle)
    }
}

impl From<(Vector3, f32)> for Vector4 {
    fn from((axis, angle): (Vector3, f32)) -> Self {
        unsafe { quaternion_from_axis_angle(axis, angle) }
    }
}

impl From<Vector4> for [f32; 4] {
    fn from(val: Vector4) -> Self {
        [val.x, val.y, val.z, val.w]
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from([x, y, z, w]: [f32; 4]) -> Self {
        Self { x, y, z, w }
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Vector4> for (f32, f32, f32, f32) {
    fn from(val: Vector4) -> Self {
        (val.x, val.y, val.z, val.w)
    }
}

impl From<(i32, i32, i32, i32)> for Vector4 {
    fn from((x, y, z, w): (i32, i32, i32, i32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
            w: w as f32,
        }
    }
}

impl From<Vector4> for (i32, i32, i32, i32) {
    fn from(val: Vector4) -> Self {
        (val.x as i32, val.y as i32, val.z as i32, val.w as i32)
    }
}

impl From<Vector4> for [i32; 4] {
    fn from(val: Vector4) -> Self {
        [val.x as i32, val.y as i32, val.z as i32, val.w as i32]
    }
}

impl From<(u32, u32, u32, u32)> for Vector4 {
    fn from((x, y, z, w): (u32, u32, u32, u32)) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
            w: w as f32,
        }
    }
}

impl From<Vector4> for (u32, u32, u32, u32) {
    fn from(val: Vector4) -> Self {
        (val.x as u32, val.y as u32, val.z as u32, val.w as u32)
    }
}

impl From<[u32; 4]> for Vector4 {
    fn from([x, y, z, w]: [u32; 4]) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            z: z as f32,
            w: w as f32,
        }
    }
}

impl From<Vector4> for [u32; 4] {
    fn from(val: Vector4) -> Self {
        [val.x as u32, val.y as u32, val.z as u32, val.w as u32]
    }
}

pub type Quaternion = Vector4;

impl Matrix {
    pub const IDENTITY: Self = Self {
        m0: 1.0,
        m4: 0.0,
        m8: 0.0,
        m12: 0.0,
        m1: 0.0,
        m5: 1.0,
        m9: 0.0,
        m13: 0.0,
        m2: 0.0,
        m6: 0.0,
        m10: 1.0,
        m14: 0.0,
        m3: 0.0,
        m7: 0.0,
        m11: 0.0,
        m15: 1.0,
    };

    pub fn determinant(&self) -> f32 {
        unsafe { matrix_determinant(*self) }
    }

    pub fn trace(&self) -> f32 {
        unsafe { matrix_trace(*self) }
    }

    pub fn transpose(&self) -> Self {
        unsafe { matrix_transpose(*self) }
    }

    pub fn invert(&self) -> Self {
        unsafe { matrix_invert(*self) }
    }

    pub fn rotate_x(degs: f32) -> Self {
        unsafe { matrix_rotate_x(degs) }
    }

    pub fn rotate_y(degs: f32) -> Self {
        unsafe { matrix_rotate_y(degs) }
    }

    pub fn rotate_z(degs: f32) -> Self {
        unsafe { matrix_rotate_z(degs) }
    }

    pub fn rotate_xyz(angles: &Vector3) -> Self {
        unsafe { matrix_rotate_xyz(*angles) }
    }

    pub fn rotate_zyx(angles: &Vector3) -> Self {
        unsafe { matrix_rotate_zyx(*angles) }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        unsafe { matrix_scale(x, y, z) }
    }

    pub fn frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        unsafe { matrix_frustum(left, right, bottom, top, near, far) }
    }

    pub fn perspective(fovy: f64, aspect: f64, near: f64, far: f64) -> Self {
        unsafe { matrix_perspective(fovy, aspect, near, far) }
    }

    pub fn ortho(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        unsafe { matrix_ortho(left, right, bottom, top, near, far) }
    }

    pub fn look_at(eye: &Vector3, target: &Vector3, up: &Vector3) -> Self {
        unsafe { matrix_look_at(*eye, *target, *up) }
    }
}

impl IntoIterator for Matrix {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.m0, self.m4, self.m8, self.m12, self.m1, self.m5, self.m9, self.m13, self.m2,
            self.m6, self.m10, self.m14, self.m3, self.m7, self.m11, self.m15,
        ]
        .into_iter()
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unsafe { matrix_add(self, rhs) }
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unsafe { matrix_subtract(self, rhs) }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe { matrix_multiply(self, rhs) }
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<Vector3> for Matrix {
    fn from(v: Vector3) -> Self {
        unsafe { matrix_translate(v.x, v.y, v.z) }
    }
}

impl From<(f32, f32, f32)> for Matrix {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        unsafe { matrix_translate(x, y, z) }
    }
}

impl From<[f32; 3]> for Matrix {
    fn from([x, y, z]: [f32; 3]) -> Self {
        unsafe { matrix_translate(x, y, z) }
    }
}

impl From<(Vector3, f32)> for Matrix {
    fn from((axis, angle): (Vector3, f32)) -> Self {
        unsafe { matrix_rotate(axis, angle) }
    }
}

impl From<Matrix> for [f32; 16] {
    fn from(val: Matrix) -> Self {
        [
            val.m0, val.m4, val.m8, val.m12, val.m1, val.m5, val.m9, val.m13, val.m2, val.m6,
            val.m10, val.m14, val.m3, val.m7, val.m11, val.m15,
        ]
    }
}

impl From<[f32; 16]> for Matrix {
    fn from(
        [m0, m4, m8, m12, m1, m5, m9, m13, m2, m6, m10, m14, m3, m7, m11, m15]: [f32; 16],
    ) -> Self {
        Self {
            m0,
            m4,
            m8,
            m12,
            m1,
            m5,
            m9,
            m13,
            m2,
            m6,
            m10,
            m14,
            m3,
            m7,
            m11,
            m15,
        }
    }
}

impl From<Matrix> for [[f32; 4]; 4] {
    fn from(val: Matrix) -> Self {
        [
            [val.m0, val.m4, val.m8, val.m12],
            [val.m1, val.m5, val.m9, val.m13],
            [val.m2, val.m6, val.m10, val.m14],
            [val.m3, val.m7, val.m11, val.m15],
        ]
    }
}

impl From<[[f32; 4]; 4]> for Matrix {
    fn from(
        [[m0, m4, m8, m12], [m1, m5, m9, m13], [m2, m6, m10, m14], [m3, m7, m11, m15]]: [[f32; 4];
            4],
    ) -> Self {
        Self {
            m0,
            m4,
            m8,
            m12,
            m1,
            m5,
            m9,
            m13,
            m2,
            m6,
            m10,
            m14,
            m3,
            m7,
            m11,
            m15,
        }
    }
}

impl From<[Vector4; 4]> for Matrix {
    fn from([v0, v1, v2, v3]: [Vector4; 4]) -> Self {
        Self {
            m0: v0.x,
            m4: v0.y,
            m8: v0.z,
            m12: v0.w,
            m1: v1.x,
            m5: v1.y,
            m9: v1.z,
            m13: v1.w,
            m2: v2.x,
            m6: v2.y,
            m10: v2.z,
            m14: v2.w,
            m3: v3.x,
            m7: v3.y,
            m11: v3.z,
            m15: v3.w,
        }
    }
}

impl From<Matrix> for [Vector4; 4] {
    fn from(val: Matrix) -> Self {
        [
            Vector4 {
                x: val.m0,
                y: val.m4,
                z: val.m8,
                w: val.m12,
            },
            Vector4 {
                x: val.m1,
                y: val.m5,
                z: val.m9,
                w: val.m13,
            },
            Vector4 {
                x: val.m2,
                y: val.m6,
                z: val.m10,
                w: val.m14,
            },
            Vector4 {
                x: val.m3,
                y: val.m7,
                z: val.m11,
                w: val.m15,
            },
        ]
    }
}

impl From<Matrix> for (Vector4, Vector4, Vector4, Vector4) {
    fn from(val: Matrix) -> Self {
        let [v0, v1, v2, v3]: [Vector4; 4] = val.into();
        (v0, v1, v2, v3)
    }
}

impl From<(Vector4, Vector4, Vector4, Vector4)> for Matrix {
    fn from((v0, v1, v2, v3): (Vector4, Vector4, Vector4, Vector4)) -> Self {
        let arr: [Vector4; 4] = [v0, v1, v2, v3];
        arr.into()
    }
}

impl BoundingBox {
    pub fn new(min: Vector3, max: Vector3) -> Self {
        Self { min, max }
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        unsafe { check_collision_boxes(*self, *other) }
    }
}
