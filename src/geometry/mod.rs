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
