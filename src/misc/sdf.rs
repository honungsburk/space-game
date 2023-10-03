use bevy::prelude::Vec2;

// Source Links
//
// https://iquilezles.org/articles/distfunctions/
// https://iquilezles.org/articles/distfunctions2d/
// https://iquilezles.org/articles/distgradfunctions2d/

////////////////////////////////////////////////////////////////////////////////
// Core
////////////////////////////////////////////////////////////////////////////////

pub trait SDF2D {
    fn distance(&self, point: Vec2) -> f32;

    // fn union(&self, other: Box<dyn SDF2D>) -> Union {
    //     Union::new(self, other)
    // }
}

////////////////////////////////////////////////////////////////////////////////
// helpers
////////////////////////////////////////////////////////////////////////////////

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

////////////////////////////////////////////////////////////////////////////////
// Operations
////////////////////////////////////////////////////////////////////////////////

// Primitive combinations

// Union

pub struct Union<A: SDF2D, B: SDF2D> {
    a: A,
    b: B,
}

impl<A: SDF2D, B: SDF2D> Union<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

pub fn union<A: SDF2D, B: SDF2D>(a: A, b: B) -> Union<A, B> {
    Union::new(a, b)
}

impl<A: SDF2D, B: SDF2D> SDF2D for Union<A, B> {
    fn distance(&self, point: Vec2) -> f32 {
        self.a.distance(point).min(self.b.distance(point))
    }
}

// subtraction

pub struct Subtraction<A: SDF2D, B: SDF2D> {
    a: A,
    b: B,
}

impl<A: SDF2D, B: SDF2D> Subtraction<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

pub fn subtraction<A: SDF2D, B: SDF2D>(a: A, b: B) -> Subtraction<A, B> {
    Subtraction::new(a, b)
}

impl<A: SDF2D, B: SDF2D> SDF2D for Subtraction<A, B> {
    fn distance(&self, point: Vec2) -> f32 {
        self.a.distance(point).max(-self.b.distance(point))
    }
}

// intersection

pub struct Intersection<A: SDF2D, B: SDF2D> {
    a: A,
    b: B,
}

impl<A: SDF2D, B: SDF2D> Intersection<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

pub fn intersection<A: SDF2D, B: SDF2D>(a: A, b: B) -> Intersection<A, B> {
    Intersection::new(a, b)
}

impl<A: SDF2D, B: SDF2D> SDF2D for Intersection<A, B> {
    fn distance(&self, point: Vec2) -> f32 {
        self.a.distance(point).max(self.b.distance(point))
    }
}

// Transformations

// Translation

pub struct Translation<T: SDF2D> {
    sdf: T,
    translation: Vec2,
}

impl<T: SDF2D> Translation<T> {
    pub fn new(sdf: T, translation: Vec2) -> Self {
        Self { sdf, translation }
    }
}

pub fn translation<T: SDF2D>(sdf: T, translation: Vec2) -> Translation<T> {
    Translation::new(sdf, translation)
}

impl<T: SDF2D> SDF2D for Translation<T> {
    fn distance(&self, point: Vec2) -> f32 {
        self.sdf.distance(point - self.translation)
    }
}

// Rotation

pub struct Rotation<T: SDF2D> {
    sdf: T,
    rotation: f32,
}

impl<T: SDF2D> Rotation<T> {
    pub fn new(sdf: T, rotation: f32) -> Self {
        Self { sdf, rotation }
    }
}

pub fn rotation<T: SDF2D>(sdf: T, rotation: f32) -> Rotation<T> {
    Rotation::new(sdf, rotation)
}

impl<T: SDF2D> SDF2D for Rotation<T> {
    fn distance(&self, point: Vec2) -> f32 {
        let c = self.rotation.cos();
        let s = self.rotation.sin();
        let q = Vec2::new(c, s);
        let r = Vec2::new(-s, c);
        self.sdf.distance(Vec2::new(point.dot(q), point.dot(r)))
    }
}

// Scaling

pub struct Scaling<T: SDF2D> {
    sdf: T,
    scale: Vec2,
}

impl<T: SDF2D> Scaling<T> {
    pub fn new(sdf: T, scale: Vec2) -> Self {
        Self { sdf, scale }
    }
}

pub fn scaling<T: SDF2D>(sdf: T, scale: Vec2) -> Scaling<T> {
    Scaling::new(sdf, scale)
}

impl<T: SDF2D> SDF2D for Scaling<T> {
    fn distance(&self, point: Vec2) -> f32 {
        self.sdf.distance(point / self.scale)
    }
}

// Smooth Union

pub struct SmoothUnion<A: SDF2D, B: SDF2D> {
    a: A,
    b: B,
    blend: f32,
}

impl<A: SDF2D, B: SDF2D> SmoothUnion<A, B> {
    pub fn new(a: A, b: B, blend: f32) -> Self {
        Self { a, b, blend }
    }
}

pub fn smooth_union<A: SDF2D, B: SDF2D>(a: A, b: B, blend: f32) -> SmoothUnion<A, B> {
    SmoothUnion::new(a, b, blend)
}

impl<A: SDF2D, B: SDF2D> SDF2D for SmoothUnion<A, B> {
    fn distance(&self, point: Vec2) -> f32 {
        let d1 = self.a.distance(point);
        let d2 = self.b.distance(point);
        let h = (0.5 + 0.5 * (d2 - d1) / self.blend).clamp(0.0, 1.0);
        mix(d2, d1, h) - self.blend * h * (1.0 - h)
    }
}

// Smooth Subtraction

pub struct SmoothSubtraction<A: SDF2D, B: SDF2D> {
    a: A,
    b: B,
    blend: f32,
}

impl<A: SDF2D, B: SDF2D> SmoothSubtraction<A, B> {
    pub fn new(a: A, b: B, blend: f32) -> Self {
        Self { a, b, blend }
    }
}

pub fn smooth_subtraction<A: SDF2D, B: SDF2D>(a: A, b: B, blend: f32) -> SmoothSubtraction<A, B> {
    SmoothSubtraction::new(a, b, blend)
}

impl<A: SDF2D, B: SDF2D> SDF2D for SmoothSubtraction<A, B> {
    fn distance(&self, point: Vec2) -> f32 {
        let d1 = self.a.distance(point);
        let d2 = self.b.distance(point);
        let h = (0.5 - 0.5 * (d2 + d1) / self.blend).clamp(0.0, 1.0);
        mix(d2, -d1, h) + self.blend * h * (1.0 - h)
    }
}

// Smooth Intersection

pub struct SmoothIntersection<A: SDF2D, B: SDF2D> {
    a: A,
    b: B,
    blend: f32,
}

impl<A: SDF2D, B: SDF2D> SmoothIntersection<A, B> {
    pub fn new(a: A, b: B, blend: f32) -> Self {
        Self { a, b, blend }
    }
}

pub fn smooth_intersection<A: SDF2D, B: SDF2D>(a: A, b: B, blend: f32) -> SmoothIntersection<A, B> {
    SmoothIntersection::new(a, b, blend)
}

impl<A: SDF2D, B: SDF2D> SDF2D for SmoothIntersection<A, B> {
    fn distance(&self, point: Vec2) -> f32 {
        let d1 = self.a.distance(point);
        let d2 = self.b.distance(point);
        let h = (0.5 - 0.5 * (d2 - d1) / self.blend).clamp(0.0, 1.0);
        mix(d2, d1, h) + self.blend * h * (1.0 - h)
    }
}

// Symmetry

// X Axis

pub struct XAxisSymmetry<T: SDF2D> {
    sdf: T,
}

impl<T: SDF2D> XAxisSymmetry<T> {
    pub fn new(sdf: T) -> Self {
        Self { sdf }
    }
}

pub fn x_axis_symmetry<T: SDF2D>(sdf: T) -> XAxisSymmetry<T> {
    XAxisSymmetry::new(sdf)
}

impl<T: SDF2D> SDF2D for XAxisSymmetry<T> {
    fn distance(&self, point: Vec2) -> f32 {
        self.sdf.distance(Vec2::new(point.x.abs(), point.y))
    }
}

// Y Axis

pub struct YAxisSymmetry<T: SDF2D> {
    sdf: T,
}

impl<T: SDF2D> YAxisSymmetry<T> {
    pub fn new(sdf: T) -> Self {
        Self { sdf }
    }
}

pub fn y_axis_symmetry<T: SDF2D>(sdf: T) -> YAxisSymmetry<T> {
    YAxisSymmetry::new(sdf)
}

impl<T: SDF2D> SDF2D for YAxisSymmetry<T> {
    fn distance(&self, point: Vec2) -> f32 {
        self.sdf.distance(Vec2::new(point.x, point.y.abs()))
    }
}

// Repetition Limited

// TODO: Implement this

// Repetition Infinite

// TODO: Implement this

// Deformation

// Displace

pub struct Displace<'a, T: SDF2D> {
    sdf: T,
    displace: &'a dyn Fn(Vec2) -> f32,
}

impl<'a, T: SDF2D> Displace<'a, T> {
    pub fn new(sdf: T, displace: &'a dyn Fn(Vec2) -> f32) -> Self {
        Self { sdf, displace }
    }
}

pub fn displace<'a, T: SDF2D>(sdf: T, displace: &'a dyn Fn(Vec2) -> f32) -> Displace<'a, T> {
    Displace::new(sdf, displace)
}

impl<'a, T: SDF2D> SDF2D for Displace<'a, T> {
    fn distance(&self, point: Vec2) -> f32 {
        self.sdf.distance(point + (self.displace)(point))
    }
}

fn displace_sin(p: Vec2) -> f32 {
    (20.0 * p.x).sin() * (20.0 * p.y).sin()
}

pub fn displace_sin_20<T: SDF2D>(sdf: T) -> Displace<'static, T> {
    Displace::new(sdf, &displace_sin)
}

////////////////////////////////////////////////////////////////////////////////
// Shapes
////////////////////////////////////////////////////////////////////////////////

// Circle

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}

impl SDF2D for Circle {
    fn distance(&self, point: Vec2) -> f32 {
        point.length() - self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_distance() {
        let circle = Circle::new(5.0);

        // Test a point outside the circle
        let point1 = Vec2::new(10.0, 0.0);
        assert_eq!(circle.distance(point1), 5.0);

        // Test a point inside the circle
        let point2 = Vec2::new(0.0, 4.0);
        assert_eq!(circle.distance(point2), -1.0);

        // Test a point on the circle
        let point3 = Vec2::new(5.0, 0.0);
        assert_eq!(circle.distance(point3), 0.0);
    }
}

// Circle
