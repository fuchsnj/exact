use std::ops::{Mul, Sub, Neg, Div, Add};
use num::{One, Zero};
use point3d::Point3d;
use vec4::Vec4;
use vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Matrix4<T> {
	x: Vec4<T>,
	y: Vec4<T>,
	z: Vec4<T>,
	w: Vec4<T>,
}

impl<T> Matrix4<T> {
	pub fn new(c0r0: T, c0r1: T, c0r2: T, c0r3: T,
	           c1r0: T, c1r1: T, c1r2: T, c1r3: T,
	           c2r0: T, c2r1: T, c2r2: T, c2r3: T,
	           c3r0: T, c3r1: T, c3r2: T, c3r3: T) -> Matrix4<T> {
		Matrix4 {
			x: Vec4::new(c0r0, c0r1, c0r2, c0r3),
			y: Vec4::new(c1r0, c1r1, c1r2, c1r3),
			z: Vec4::new(c2r0, c2r1, c2r2, c2r3),
			w: Vec4::new(c3r0, c3r1, c3r2, c3r3)
		}
	}
}

impl<T: Clone + Div<Output=T> + Mul<Output=T> + Add<Output=T> + One> Matrix4<T> {
	pub fn transform_point(&self, point: Point3d<T>) -> Point3d<T> {
		Point3d::from_homogeneous(self.clone() * point.to_homogeneous())
	}
}

impl<T: One + Zero + Clone> Matrix4<T> {
	pub fn from_translation(v: Vec3<T>) -> Matrix4<T> {
		Matrix4::new(T::one(), T::zero(), T::zero(), T::zero(),
		             T::zero(), T::one(), T::zero(), T::zero(),
		             T::zero(), T::zero(), T::one(), T::zero(),
		             v.get_x(), v.get_y(), v.get_z(), T::one())
	}

	pub fn from_scale(x: T) -> Matrix4<T> {
		Matrix4::from_scale3(x.clone(), x.clone(), x)
	}

	pub fn from_scale3(x: T, y: T, z: T) -> Matrix4<T> {
		Matrix4::new(x, T::zero(), T::zero(), T::zero(),
		             T::zero(), y, T::zero(), T::zero(),
		             T::zero(), T::zero(), z, T::zero(),
		             T::zero(), T::zero(), T::zero(), T::one())
	}
}

impl<T: Clone + Mul<Output=T> + Add<Output=T>> Mul for Matrix4<T> {
	type Output = Self;

	fn mul(self, b: Self) -> Self {
		Matrix4::new(
			self.x.dot(&b.x), self.y.dot(&b.x), self.z.dot(&b.x), self.w.dot(&b.x),
			self.x.dot(&b.y), self.y.dot(&b.y), self.z.dot(&b.y), self.w.dot(&b.y),
			self.x.dot(&b.z), self.y.dot(&b.z), self.z.dot(&b.z), self.w.dot(&b.z),
			self.x.dot(&b.w), self.y.dot(&b.w), self.z.dot(&b.w), self.w.dot(&b.w)
		)
	}
}

impl<T: Clone + Mul<Output=T> + Add<Output=T>> Mul<Vec4<T>> for Matrix4<T> {
	type Output = Vec4<T>;

	fn mul(self, v: Vec4<T>) -> Vec4<T> {
		self.x * v.get_x() + self.y * v.get_y() + self.z * v.get_z() + self.w * v.get_w()
	}
}