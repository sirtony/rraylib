use crate::sys::*;
use std::ops::*;
use std::ptr::addr_of_mut;

impl Vector2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn addf(&mut self, value: f32) {
        *self = unsafe { Vector2AddValue(*self, value) }
    }

    pub fn subf(&mut self, value: f32) {
        *self = unsafe { Vector2SubtractValue(*self, value) }
    }

    pub fn scale(&mut self, value: f32) {
        *self = unsafe { Vector2Scale(*self, value) }
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn len(&self) -> f32 {
        unsafe { Vector2Length(*self) }
    }

    pub fn len_squared(&self) -> f32 {
        unsafe { Vector2LengthSqr(*self) }
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
        unsafe { Vector2DotProduct(*self, *other) }
    }

    pub fn distance(&self, other: &Vector2) -> f32 {
        unsafe { Vector2Distance(*self, *other) }
    }

    pub fn distance_squared(&self, other: &Vector2) -> f32 {
        unsafe { Vector2DistanceSqr(*self, *other) }
    }

    pub fn angle(&self, other: &Vector2) -> f32 {
        unsafe { Vector2Angle(*self, *other) }
    }

    pub fn normalize(&self) -> Self {
        unsafe { Vector2Normalize(*self) }
    }

    pub fn transform(&self, mat: &Matrix) -> Self {
        unsafe { Vector2Transform(*self, *mat) }
    }

    pub fn lerp(&self, target: &Vector2, amount: f32) -> Self {
        unsafe { Vector2Lerp(*self, *target, amount) }
    }

    pub fn reflect(&self, normal: &Vector2) -> Self {
        unsafe { Vector2Reflect(*self, *normal) }
    }

    pub fn rotate(&self, degs: f32) -> Self {
        unsafe { Vector2Rotate(*self, degs) }
    }

    pub fn move_towards(&self, target: &Vector2, max_distance: f32) -> Self {
        unsafe { Vector2MoveTowards(*self, *target, max_distance) }
    }

    pub fn invert(&self) -> Self {
        unsafe { Vector2Invert(*self) }
    }

    pub fn clamp(&self, min: &Vector2, max: &Vector2) -> Self {
        unsafe { Vector2Clamp(*self, *min, *max) }
    }

    pub fn clampf(&self, min: f32, max: f32) -> Self {
        unsafe { Vector2ClampValue(*self, min, max) }
    }
}

impl Eq for Vector2 {}
impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Vector2Equals(*self, *other) == 1 }
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
        unsafe { Vector2Add(self, rhs) }
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
        unsafe { Vector2Subtract(self, rhs) }
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
        unsafe { Vector2Multiply(self, rhs) }
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
        unsafe { Vector2Divide(self, rhs) }
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
        unsafe { Vector2Negate(self) }
    }
}

impl Into<[f32; 2]> for Vector2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
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

impl Into<(f32, f32)> for Vector2 {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
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

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn addf(&mut self, value: f32) {
        *self = unsafe { Vector3AddValue(*self, value) }
    }

    pub fn subf(&mut self, value: f32) {
        *self = unsafe { Vector3SubtractValue(*self, value) }
    }

    pub fn scale(&mut self, value: f32) {
        *self = unsafe { Vector3Scale(*self, value) }
    }

    pub fn cross(&self, other: &Vector3) -> Self {
        unsafe { Vector3CrossProduct(*self, *other) }
    }

    pub fn perp(&self) -> Self {
        unsafe { Vector3Perpendicular(*self) }
    }

    pub fn len(&self) -> f32 {
        unsafe { Vector3Length(*self) }
    }

    pub fn len_squared(&self) -> f32 {
        unsafe { Vector3LengthSqr(*self) }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        unsafe { Vector3DotProduct(*self, *other) }
    }

    pub fn distance(&self, other: &Vector3) -> f32 {
        unsafe { Vector3Distance(*self, *other) }
    }

    pub fn distance_squared(&self, other: &Vector3) -> f32 {
        unsafe { Vector3DistanceSqr(*self, *other) }
    }

    pub fn angle(&self, other: &Vector3) -> f32 {
        unsafe { Vector3Angle(*self, *other) }
    }

    pub fn normalize(&self) -> Self {
        unsafe { Vector3Normalize(*self) }
    }

    pub fn transform(&self, mat: &Matrix) -> Self {
        unsafe { Vector3Transform(*self, *mat) }
    }

    pub fn quat_rotate(&self, q: &Quaternion) -> Self {
        unsafe { Vector3RotateByQuaternion(*self, *q) }
    }

    pub fn rotate(&self, axis: &Vector3, degs: f32) -> Self {
        unsafe { Vector3RotateByAxisAngle(*self, *axis, degs) }
    }

    pub fn lerp(&self, target: &Vector3, amount: f32) -> Self {
        unsafe { Vector3Lerp(*self, *target, amount) }
    }

    pub fn reflect(&self, normal: &Vector3) -> Self {
        unsafe { Vector3Reflect(*self, *normal) }
    }

    pub fn min(&self, other: &Vector3) -> Self {
        unsafe { Vector3Min(*self, *other) }
    }

    pub fn max(&self, other: &Vector3) -> Self {
        unsafe { Vector3Max(*self, *other) }
    }

    pub fn barycenter(&self, i: &Vector3, j: &Vector3, k: &Vector3) -> Self {
        unsafe { Vector3Barycenter(*self, *i, *j, *k) }
    }

    pub fn unproject(&self, proj: &Matrix, view: &Matrix) -> Self {
        unsafe { Vector3Unproject(*self, *proj, *view) }
    }

    pub fn invert(&self) -> Self {
        unsafe { Vector3Invert(*self) }
    }

    pub fn clamp(&self, min: &Vector3, max: &Vector3) -> Self {
        unsafe { Vector3Clamp(*self, *min, *max) }
    }

    pub fn clampf(&self, min: f32, max: f32) -> Self {
        unsafe { Vector3ClampValue(*self, min, max) }
    }

    pub fn refract(&self, normal: &Vector3, ior: f32) -> Self {
        unsafe { Vector3Refract(*self, *normal, ior) }
    }
}

impl Eq for Vector3 {}
impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Vector3Equals(*self, *other) == 1 }
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
        unsafe { Vector3Add(self, rhs) }
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
        unsafe { Vector3Subtract(self, rhs) }
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
        unsafe { Vector3Multiply(self, rhs) }
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
        unsafe { Vector3Divide(self, rhs) }
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
        unsafe { Vector3Negate(self) }
    }
}

impl Into<[f32; 3]> for Vector3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
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

impl Into<(f32, f32, f32)> for Vector3 {
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
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
        unsafe { QuaternionFromAxisAngle(*axis, angle) }
    }

    pub fn identity() -> Self {
        unsafe { QuaternionIdentity() }
    }

    pub fn addf(&mut self, value: f32) {
        *self = unsafe { Vector4AddValue(*self, value) }
    }

    pub fn subf(&mut self, value: f32) {
        *self = unsafe { Vector4SubtractValue(*self, value) }
    }

    pub fn scale(&mut self, value: f32) {
        *self = unsafe { Vector4Scale(*self, value) }
    }

    pub fn len(&self) -> f32 {
        unsafe { Vector4Length(*self) }
    }

    pub fn len_squared(&self) -> f32 {
        unsafe { Vector4LengthSqr(*self) }
    }

    pub fn normalize(&self) -> Self {
        unsafe { Vector4Normalize(*self) }
    }

    pub fn dot(&self, other: &Vector4) -> f32 {
        unsafe { Vector4DotProduct(*self, *other) }
    }

    pub fn lerp(&self, target: &Vector4, amount: f32) -> Self {
        unsafe { Vector4Lerp(*self, *target, amount) }
    }

    pub fn nlerp(&self, target: &Vector4, amount: f32) -> Self {
        unsafe { QuaternionNlerp(*self, *target, amount) }
    }

    pub fn slerp(&self, target: &Vector4, amount: f32) -> Self {
        unsafe { QuaternionSlerp(*self, *target, amount) }
    }

    pub fn invert(&self) -> Self {
        unsafe { Vector4Invert(*self) }
    }

    pub fn move_towards(&self, target: &Vector4, max_distance: f32) -> Self {
        unsafe { Vector4MoveTowards(*self, *target, max_distance) }
    }

    pub fn distance(&self, other: &Vector4) -> f32 {
        unsafe { Vector4Distance(*self, *other) }
    }

    pub fn min(&self, other: &Vector4) -> Self {
        unsafe { Vector4Min(*self, *other) }
    }

    pub fn max(&self, other: &Vector4) -> Self {
        unsafe { Vector4Max(*self, *other) }
    }

    pub fn transform(&self, mat: &Matrix) -> Self {
        unsafe { QuaternionTransform(*self, *mat) }
    }
}

impl Eq for Vector4 {}
impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Vector4Equals(*self, *other) == 1 }
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
        unsafe { Vector4Add(self, rhs) }
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
        unsafe { Vector4Subtract(self, rhs) }
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
        unsafe { Vector4Multiply(self, rhs) }
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
        unsafe { Vector4Divide(self, rhs) }
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
        unsafe { Vector4Negate(self) }
    }
}

impl Into<Vector3> for Vector4 {
    fn into(self) -> Vector3 {
        unsafe { QuaternionToEuler(self) }
    }
}

impl From<(Vector3, Vector3)> for Vector4 {
    fn from((from, to): (Vector3, Vector3)) -> Self {
        unsafe { QuaternionFromVector3ToVector3(from, to) }
    }
}

impl From<[Vector3; 2]> for Vector4 {
    fn from([from, to]: [Vector3; 2]) -> Self {
        unsafe { QuaternionFromVector3ToVector3(from, to) }
    }
}

impl Into<Matrix> for Vector4 {
    fn into(self) -> Matrix {
        unsafe { QuaternionToMatrix(self) }
    }
}

impl Into<(Vector3, f32)> for Vector4 {
    fn into(self) -> (Vector3, f32) {
        let mut axis = Vector3::ZERO;
        let mut angle = 0.0;
        unsafe { QuaternionToAxisAngle(self, addr_of_mut!(axis), addr_of_mut!(angle)) }

        (axis, angle)
    }
}

impl From<(Vector3, f32)> for Vector4 {
    fn from((axis, angle): (Vector3, f32)) -> Self {
        unsafe { QuaternionFromAxisAngle(axis, angle) }
    }
}

impl Into<[f32; 4]> for Vector4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
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

impl Into<(f32, f32, f32, f32)> for Vector4 {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
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
        unsafe { MatrixDeterminant(*self) }
    }

    pub fn trace(&self) -> f32 {
        unsafe { MatrixTrace(*self) }
    }

    pub fn transpose(&self) -> Self {
        unsafe { MatrixTranspose(*self) }
    }

    pub fn invert(&self) -> Self {
        unsafe { MatrixInvert(*self) }
    }

    pub fn rotate_x(degs: f32) -> Self {
        unsafe { MatrixRotateX(degs) }
    }

    pub fn rotate_y(degs: f32) -> Self {
        unsafe { MatrixRotateY(degs) }
    }

    pub fn rotate_z(degs: f32) -> Self {
        unsafe { MatrixRotateZ(degs) }
    }

    pub fn rotate_xyz(angles: &Vector3) -> Self {
        unsafe { MatrixRotateXYZ(*angles) }
    }

    pub fn rotate_zyx(angles: &Vector3) -> Self {
        unsafe { MatrixRotateZYX(*angles) }
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        unsafe { MatrixScale(x, y, z) }
    }

    pub fn frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        unsafe { MatrixFrustum(left, right, bottom, top, near, far) }
    }

    pub fn perspective(fovy: f64, aspect: f64, near: f64, far: f64) -> Self {
        unsafe { MatrixPerspective(fovy, aspect, near, far) }
    }

    pub fn ortho(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        unsafe { MatrixOrtho(left, right, bottom, top, near, far) }
    }

    pub fn look_at(eye: &Vector3, target: &Vector3, up: &Vector3) -> Self {
        unsafe { MatrixLookAt(*eye, *target, *up) }
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
        unsafe { MatrixAdd(self, rhs) }
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
        unsafe { MatrixSubtract(self, rhs) }
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
        unsafe { MatrixMultiply(self, rhs) }
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl From<Vector3> for Matrix {
    fn from(v: Vector3) -> Self {
        unsafe { MatrixTranslate(v.x, v.y, v.z) }
    }
}

impl From<(f32, f32, f32)> for Matrix {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        unsafe { MatrixTranslate(x, y, z) }
    }
}

impl From<[f32; 3]> for Matrix {
    fn from([x, y, z]: [f32; 3]) -> Self {
        unsafe { MatrixTranslate(x, y, z) }
    }
}

impl From<(Vector3, f32)> for Matrix {
    fn from((axis, angle): (Vector3, f32)) -> Self {
        unsafe { MatrixRotate(axis, angle) }
    }
}

impl Into<[f32; 16]> for Matrix {
    fn into(self) -> [f32; 16] {
        [
            self.m0, self.m4, self.m8, self.m12, self.m1, self.m5, self.m9, self.m13, self.m2,
            self.m6, self.m10, self.m14, self.m3, self.m7, self.m11, self.m15,
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

impl Into<[[f32; 4]; 4]> for Matrix {
    fn into(self) -> [[f32; 4]; 4] {
        [
            [self.m0, self.m4, self.m8, self.m12],
            [self.m1, self.m5, self.m9, self.m13],
            [self.m2, self.m6, self.m10, self.m14],
            [self.m3, self.m7, self.m11, self.m15],
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

impl Into<[Vector4; 4]> for Matrix {
    fn into(self) -> [Vector4; 4] {
        [
            Vector4 {
                x: self.m0,
                y: self.m4,
                z: self.m8,
                w: self.m12,
            },
            Vector4 {
                x: self.m1,
                y: self.m5,
                z: self.m9,
                w: self.m13,
            },
            Vector4 {
                x: self.m2,
                y: self.m6,
                z: self.m10,
                w: self.m14,
            },
            Vector4 {
                x: self.m3,
                y: self.m7,
                z: self.m11,
                w: self.m15,
            },
        ]
    }
}

impl Into<(Vector4, Vector4, Vector4, Vector4)> for Matrix {
    fn into(self) -> (Vector4, Vector4, Vector4, Vector4) {
        let [v0, v1, v2, v3]: [Vector4; 4] = self.into();
        (v0, v1, v2, v3)
    }
}

impl From<(Vector4, Vector4, Vector4, Vector4)> for Matrix {
    fn from((v0, v1, v2, v3): (Vector4, Vector4, Vector4, Vector4)) -> Self {
        let arr: [Vector4; 4] = [v0, v1, v2, v3];
        arr.into()
    }
}
