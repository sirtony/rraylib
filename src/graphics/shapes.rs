use crate::graphics::Color;
use crate::math::{Rectangle, Vector2, Vector3};
use crate::sys::*;
use std::f64::consts::PI;
use std::ptr::addr_of_mut;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SplineType {
    Linear,
    Basis,
    CatmullRom,
    Quadratic,
    Cubic,
}

impl SplineType {
    pub(crate) fn draw(
        &self,
        points: impl ExactSizeIterator<Item = Vector2>,
        thickness: f32,
        color: impl Into<Color>,
    ) {
        let color = color.into();
        let points: Vec<Vector2> = points.collect();
        let ptr = points.as_ptr();

        match self {
            SplineType::Linear => unsafe {
                draw_spline_linear(ptr, points.len() as i32, thickness, color)
            },
            SplineType::Basis => unsafe {
                draw_spline_basis(ptr, points.len() as i32, thickness, color)
            },
            SplineType::CatmullRom => unsafe {
                draw_spline_catmull_rom(ptr, points.len() as i32, thickness, color)
            },
            SplineType::Quadratic => unsafe {
                draw_spline_bezier_quadratic(ptr, points.len() as i32, thickness, color)
            },
            SplineType::Cubic => unsafe {
                draw_spline_bezier_cubic(ptr, points.len() as i32, thickness, color)
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shape3D<'t> {
    Line(Vector3, Vector3),
    Circle {
        center: Vector3,
        radius: f32,
        rotation_axis: Vector3,
        rotation_angle: f32,
    },
    Triangle(Vector3, Vector3, Vector3),
    TriangleStrip(&'t [Vector3]),
    Cube {
        position: Vector3,
        size: Vector3,
    },
    Sphere {
        center: Vector3,
        radius: f32,
        rings: u32,
        slices: u32,
    },
    Cylinder {
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        slices: u32,
    },
    Capsule {
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: u32,
        rings: u32,
    },
    Plane {
        center: Vector3,
        size: Vector2,
    },
    Grid {
        slices: u32,
        spacing: f32,
    },
    Ray(Ray),
}

impl Shape3D<'_> {
    pub fn collides_with<'t2>(&self, other: impl Into<Shape3D<'t2>>) -> bool {
        let pair = (*self, other.into());
        match pair {
            (
                Shape3D::Sphere {
                    center: c1,
                    radius: r1,
                    ..
                },
                Shape3D::Sphere {
                    center: c2,
                    radius: r2,
                    ..
                },
            ) => unsafe { check_collision_spheres(c1, r1, c2, r2) },

            _ => false,
        }
    }

    pub fn collides_with_box(&self, bounding_box: impl Into<BoundingBox>) -> bool {
        match *self {
            Shape3D::Sphere { center, radius, .. } => unsafe {
                check_collision_box_sphere(bounding_box.into(), center, radius)
            },
            _ => false,
        }
    }
}

impl From<Ray> for Shape3D<'_> {
    fn from(ray: Ray) -> Self {
        Self::Ray(ray)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Shape2D<'t> {
    Pixel(Vector2),
    Line {
        start: Vector2,
        end: Vector2,
        thickness: Option<f32>,
    },
    LineStrip(&'t [Vector2]),
    Bezier {
        start: Vector2,
        end: Vector2,
        thickness: f32,
    },
    Circle {
        center: Vector2,
        radius: f32,
    },
    Pie {
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: u32,
    },
    Ellipse {
        center: Vector2,
        radius: Vector2,
    },
    Ring {
        center: Vector2,
        inner_radius: f32,
        outer_radius: f32,
        angle: Vector2,
        segments: u32,
    },
    Rectangle {
        rect: Rectangle,
        rotation: Option<f32>,
    },
    RoundedRectangle {
        rect: Rectangle,
        roundness: f32,
        segments: i32,
    },
    Triangle {
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
    },
    TriangleFan(&'t [Vector2]),
    TriangleStrip(&'t [Vector2]),
    Polygon {
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
    },
    Spline {
        spline_type: SplineType,
        points: &'t [Vector2],
        thickness: f32,
    },
}

impl<'t> Shape2D<'t> {
    pub fn collides_with(&self, other: impl Into<Shape2D<'t>>) -> bool {
        let other = other.into();
        let pair = (self, &other);

        match pair {
            (Shape2D::Rectangle { rect: r1, .. }, Shape2D::Rectangle { rect: r2, .. }) => unsafe {
                check_collision_recs(*r1, *r2)
            },
            (
                Shape2D::Circle {
                    center: c1,
                    radius: r1,
                },
                Shape2D::Circle {
                    center: c2,
                    radius: r2,
                },
            ) => unsafe { check_collision_circles(*c1, *r1, *c2, *r2) },
            (
                Shape2D::Rectangle { rect: r, .. },
                Shape2D::Circle {
                    center: c,
                    radius: r2,
                },
            ) => unsafe { check_collision_circle_rec(*c, *r2, *r) },
            (
                Shape2D::Circle {
                    center: c,
                    radius: r,
                },
                Shape2D::Rectangle { rect: rc, .. },
            ) => unsafe { check_collision_circle_rec(*c, *r, *rc) },

            (
                Shape2D::Circle {
                    center: c,
                    radius: r,
                },
                Shape2D::Line {
                    start: s, end: e, ..
                },
            ) => unsafe { check_collision_circle_line(*c, *r, *s, *e) },
            (
                Shape2D::Line {
                    start: s, end: e, ..
                },
                Shape2D::Circle {
                    center: c,
                    radius: r,
                },
            ) => unsafe { check_collision_circle_line(*c, *r, *s, *e) },

            (Shape2D::Pixel(p), Shape2D::Rectangle { rect: r, .. }) => unsafe {
                check_collision_point_rec(*p, *r)
            },
            (Shape2D::Rectangle { rect: r, .. }, Shape2D::Pixel(p)) => unsafe {
                check_collision_point_rec(*p, *r)
            },

            (
                Shape2D::Pixel(p),
                Shape2D::Circle {
                    center: c,
                    radius: r,
                },
            ) => unsafe { check_collision_point_circle(*p, *c, *r) },
            (
                Shape2D::Circle {
                    center: c,
                    radius: r,
                },
                Shape2D::Pixel(p),
            ) => unsafe { check_collision_point_circle(*p, *c, *r) },

            (Shape2D::Pixel(p), Shape2D::Triangle { v1, v2, v3 }) => unsafe {
                check_collision_point_triangle(*p, *v1, *v2, *v3)
            },
            (Shape2D::Triangle { v1, v2, v3 }, Shape2D::Pixel(p)) => unsafe {
                check_collision_point_triangle(*p, *v1, *v2, *v3)
            },

            (
                Shape2D::Pixel(p),
                Shape2D::Line {
                    start: s, end: e, ..
                },
            ) => unsafe { check_collision_point_line(*p, *s, *e, 1) },
            (
                Shape2D::Line {
                    start: s, end: e, ..
                },
                Shape2D::Pixel(p),
            ) => unsafe { check_collision_point_line(*p, *s, *e, 1) },

            (
                Shape2D::Pixel(p),
                Shape2D::Polygon {
                    center,
                    sides,
                    radius,
                    ..
                },
            )
            | (
                Shape2D::Polygon {
                    center,
                    sides,
                    radius,
                    ..
                },
                Shape2D::Pixel(p),
            ) => {
                let mut points = Vec::with_capacity(*sides as usize);
                for i in 0..*sides {
                    let radius = *radius as f64;
                    let x = center.x as f64;
                    let y = center.y as f64;
                    let angle = 2.0 * PI / *sides as f64 * i as f64;
                    let x = x + radius * angle.cos();
                    let y = y + radius * angle.sin();
                    points.push(Vector2::new(x as f32, y as f32));
                }

                unsafe { check_collision_point_poly(*p, points.as_ptr(), points.len() as i32) }
            }

            _ => false,
        }
    }

    pub fn collision_point(&self, other: impl Into<Shape2D<'t>>) -> Option<(i32, i32)> {
        let (x, y): (i32, i32) = self.collision_pointv(other)?.into();
        Some((x, y))
    }

    pub fn collision_pointv(&self, other: impl Into<Shape2D<'t>>) -> Option<Vector2> {
        let other = other.into();
        let pair = (self, &other);

        match pair {
            (
                Shape2D::Line {
                    start: s1, end: e1, ..
                },
                Shape2D::Line {
                    start: s2, end: e2, ..
                },
            ) => {
                let mut ptr = Vector2::ZERO;
                let has_collision =
                    unsafe { check_collision_lines(*s1, *e1, *s2, *e2, addr_of_mut!(ptr)) };

                if has_collision {
                    Some(ptr)
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    pub fn spline_point(&self, t: f32) -> Option<Vector2> {
        let t = t.clamp(0.0, 1.0);
        match self {
            Shape2D::Spline {
                spline_type,
                points,
                ..
            } => match spline_type {
                SplineType::Linear => {
                    let p1 = points.first()?;
                    let p2 = points.get(1)?;
                    Some(unsafe { get_spline_point_linear(*p1, *p2, t) })
                }
                SplineType::Basis => {
                    let p1 = points.first()?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    let p4 = points.get(3)?;
                    Some(unsafe { get_spline_point_basis(*p1, *p2, *p3, *p4, t) })
                }
                SplineType::CatmullRom => {
                    let p1 = points.first()?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    let p4 = points.get(3)?;
                    Some(unsafe { get_spline_point_catmull_rom(*p1, *p2, *p3, *p4, t) })
                }
                SplineType::Quadratic => {
                    let p1 = points.first()?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    Some(unsafe { get_spline_point_bezier_quad(*p1, *p2, *p3, t) })
                }
                SplineType::Cubic => {
                    let p1 = points.first()?;
                    let p2 = points.get(1)?;
                    let p3 = points.get(2)?;
                    let p4 = points.get(3)?;
                    Some(unsafe { get_spline_point_bezier_cubic(*p1, *p2, *p3, *p4, t) })
                }
            },
            _ => None,
        }
    }
}

impl<T: Into<Rectangle>> From<T> for Shape2D<'_> {
    fn from(rect: T) -> Self {
        Self::Rectangle {
            rect: rect.into(),
            rotation: None,
        }
    }
}
