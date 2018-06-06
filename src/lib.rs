//! aobench: Ambient Occlusion Renderer benchmark.
//!
//! Based on: https://code.google.com/archive/p/aobench/

#![feature(stdsimd, iterator_step_by)]
#![allow(non_snake_case, non_camel_case_types)]

#[macro_use]
extern crate cfg_if;
extern crate failure;
extern crate png;
extern crate rayon;
extern crate stdsimd;

pub mod ambient_occlusion;
pub mod geometry;
pub mod image;
pub mod intersection;
pub mod random;
pub mod scene;

pub mod scalar;
pub mod scalar_parallel;
pub mod vector;
pub mod vector_parallel;

pub use self::image::Image;
pub use self::scene::Scene;
