use std::ops::{Mul, Sub, Neg, Div, Add};
use exact::Sqrt;


#[derive(Clone, Debug)]
pub struct Vec4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T
}

impl<T> Vec4<T> {
	pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
		Vec4 { x, y, z, w }
	}
}

impl<T: Clone> Vec4<T> {
	pub fn get_x(&self) -> T {
		self.x.clone()
	}

	pub fn get_y(&self) -> T {
		self.y.clone()
	}

	pub fn get_z(&self) -> T {
		self.z.clone()
	}

	pub fn get_w(&self) -> T { self.w.clone() }
}

impl<T: Clone + Mul<Output=T> + Add<Output=T>> Vec4<T> {
	pub fn dot(&self, other: &Vec4<T>) -> T {
		self.get_x() * other.get_x()
				+ self.get_y() * other.get_y()
				+ self.get_z() * other.get_z()
				+ self.get_w() * other.get_w()
	}
}