//! Intersection functions

/// Intersection of `I` with `Self`
pub trait Intersect<I> {
    type Isect;
    fn intersect(&self, other: &I, isect: Self::Isect) -> Self::Isect;
}

mod ray_plane;
mod ray_sphere;
mod single;
mod packet;

pub use self::single::Isect;
pub use self::packet::IsectxN;
