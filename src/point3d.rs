use std::ops::{Mul, Sub, Div, Add};
use num::One;
use vec4::Vec4;
use vec3::Vec3;


#[derive(Clone, Debug)]
pub struct Point3d<T> {
	pub x: T,
	pub y: T,
	pub z: T
}

impl<T> Point3d<T> {
	pub fn new(x: T, y: T, z: T) -> Point3d<T> {
		Point3d { x, y, z }
	}
}

impl<T: Clone> Point3d<T> {
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

impl<T: Clone + One> Point3d<T> {
	pub fn to_homogeneous(&self) -> Vec4<T> {
		Vec4::new(self.get_x(), self.get_y(), self.get_z(), T::one())
	}
}

impl<T: Clone + Div<Output=T>> Point3d<T> {
	pub fn from_homogeneous(v: Vec4<T>) -> Point3d<T> {
		v.truncate() / v.get_w()
	}
}

/******************** TRAITS *********************/

impl<T: Add<Output=T> + Clone> Add<Vec3<T>> for Point3d<T> {
	type Output = Point3d<T>;

	fn add(self, other: Vec3<T>) -> Point3d<T> {
		Point3d::new(
			self.get_x() + other.get_x(),
			self.get_y() + other.get_y(),
			self.get_z() + other.get_z()
		)
	}
}

impl<T: Sub<Output=T> + Clone> Sub for Point3d<T> {
	type Output = Vec3<T>;

	fn sub(self, other: Point3d<T>) -> Vec3<T> {
		Vec3::new(
			self.get_x() - other.get_x(),
			self.get_y() - other.get_y(),
			self.get_z() - other.get_z()
		)
	}
}

impl<T: Mul<Output=T> + Clone> Mul<T> for Point3d<T> {
	type Output = Point3d<T>;

	fn mul(self, x: T) -> Point3d<T> {
		Point3d::new(
			self.get_x() * x.clone(),
			self.get_y() * x.clone(),
			self.get_z() * x
		)
	}
}

impl<T: Div<Output=T> + Clone> Div<T> for Point3d<T> {
	type Output = Point3d<T>;

	fn div(self, x: T) -> Point3d<T> {
		Point3d::new(
			self.get_x() / x.clone(),
			self.get_y() / x.clone(),
			self.get_z() / x
		)
	}
}