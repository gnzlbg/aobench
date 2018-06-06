//! Geometry utilities

use stdsimd::simd::*;

mod plane;
mod ray;
mod sphere;
mod vec;

mod rayxN;
mod vecxN;

pub use self::plane::Plane;
pub use self::ray::Ray;
pub use self::sphere::Sphere;
pub use self::vec::{Dot, M3x3, V3D};

pub use self::rayxN::RayxN;
pub use self::vecxN::{Selectable, V3DxN};

pub type f32xN = f32x8;
pub type u32xN = u32x8;
pub type m32xN = m32x8;

pub trait IncrV {
    type Element;
    fn incr(x: Self::Element) -> Self;
}

impl IncrV for f32xN {
    type Element = f32;
    #[inline(always)]
    fn incr(x: f32) -> Self {
        Self::new(x, x + 1., x + 2., x + 3., x + 4., x + 5., x + 6., x + 7.)
    }
}

impl IncrV for u32xN {
    type Element = u32;
    #[inline(always)]
    fn incr(x: u32) -> Self {
        Self::new(x, x + 1, x + 2, x + 3, x + 4, x + 5, x + 6, x + 7)
    }
}

