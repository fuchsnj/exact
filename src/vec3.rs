use std::ops::{Mul, Sub, Neg, Div, Add};
use exact::Sqrt;
use num::One;
use vec4::Vec4;


#[derive(Clone, Debug)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T
}

impl<T> Vec3<T> {
	pub fn new(x: T, y: T, z: T) -> Vec3<T> {
		Vec3 { x, y, z }
	}
}

impl<T: Clone> Vec3<T> {
	pub fn get_x(&self) -> T {
		self.x.clone()
	}

	pub fn get_y(&self) -> T {
		self.y.clone()
	}

	pub fn get_z(&self) -> T {
		self.z.clone()
	}
}

impl<T: Clone + Mul<Output=T> + Sub<Output=T>> Vec3<T> {
	pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
		Vec3::new((self.get_y() * other.get_z()) - (self.get_z() * other.get_y()),
		             (self.get_z() * other.get_x()) - (self.get_x() * other.get_z()),
		             (self.get_x() * other.get_y()) - (self.get_y() * other.get_x()))
	}
}

impl<T: Clone + Mul<Output=T> + Add<Output=T>> Vec3<T> {
	pub fn dot(self, other: Vec3<T>) -> T {
		self.get_x() * other.get_x() + self.get_y() * other.get_y() + self.get_z() * other.get_z()
	}
}

impl<T: Clone + Mul<Output=T> + Add<Output=T> + Sqrt<Output=T>> Vec3<T> {
	pub fn magnitude(self) -> T {
		self.clone().dot(self).sqrt()
	}
}

impl<T: Clone + Mul<Output=T> + Add<Output=T> + Sqrt<Output=T> + Div<Output=T>> Vec3<T> {
	pub fn normalize(self) -> Vec3<T> {
		self.clone() / self.magnitude()
	}
}

impl<T: Clone + Mul<Output=T>> Vec3<T> {
	pub fn mul_element_wise(self, other: Vec3<T>) -> Vec3<T> {
		Vec3::new(
			self.get_x() * other.get_x(),
			self.get_y() * other.get_y(),
			self.get_z() * other.get_z()
		)
	}
}

/******************** TRAITS *********************/

impl<T: Add<Output=T> + Clone> Add for Vec3<T> {
	type Output = Vec3<T>;

	fn add(self, other: Vec3<T>) -> Vec3<T> {
		Vec3::new(
			self.get_x() + other.get_x(),
			self.get_y() + other.get_y(),
			self.get_z() + other.get_z()
		)
	}
}

impl<T: Sub<Output=T> + Clone> Sub for Vec3<T> {
	type Output = Vec3<T>;

	fn sub(self, other: Vec3<T>) -> Vec3<T> {
		Vec3::new(
			self.get_x() - other.get_x(),
			self.get_y() - other.get_y(),
			self.get_z() - other.get_z()
		)
	}
}

impl<T: Mul<Output=T> + Clone> Mul<T> for Vec3<T> {
	type Output = Vec3<T>;

	fn mul(self, x: T) -> Vec3<T> {
		Vec3::new(
			self.get_x() * x.clone(),
			self.get_y() * x.clone(),
			self.get_z() * x
		)
	}
}

impl<T: Div<Output=T> + Clone> Div<T> for Vec3<T> {
	type Output = Vec3<T>;

	fn div(self, x: T) -> Vec3<T> {
		Vec3::new(
			self.get_x() / x.clone(),
			self.get_y() / x.clone(),
			self.get_z() / x
		)
	}
}