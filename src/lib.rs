extern crate num;

#[cfg(test)]
mod test;

pub mod vec3;
pub mod vec4;
pub mod exact;
pub mod matrix4;

pub use exact::Exact;
pub use vec3::Vec3;

use num::{BigInt, BigUint, BigRational, ToPrimitive};
use std::ops::{Add, Mul, Sub, Div, Neg};
use std::f64;

use exact::{Sqrt};