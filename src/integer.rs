use num::{BigUint, ToPrimitive, Zero, One, BigInt, Signed};
use exact::NeedsParens;
use std::fmt::Display;
use std::fmt;
use num::bigint::Sign;
use std::ops::{Neg, Add, Mul, Div};
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Integer {
	value: BigInt
}

impl Integer {
	pub fn from_i64(x: i64) -> Integer {
		Integer { value: BigInt::from(x) }
	}
	pub fn is_negative(&self) -> bool {
		self.value.sign() == Sign::Minus
	}
	pub fn is_positive(&self) -> bool {
		!self.is_negative()
	}
	pub fn is_one(&self) -> bool {
		self.value == BigInt::from(1)
	}
	pub fn abs(&self) -> Self {
		Integer {
			value: self.value.abs()
		}
	}
	pub fn gcd(&self, other: &Self) -> Self {
		let a = self.value.abs().to_biguint().unwrap();
		let b = other.value.abs().to_biguint().unwrap();
		Integer {
			value: BigInt::from(gcd(&a, &b))
		}
	}
}

impl Neg for Integer {
	type Output = Self;

	fn neg(self) -> Self {
		Integer {
			value: -self.value
		}
	}
}

impl Add for Integer {
	type Output = Integer;

	fn add(self, other: Integer) -> Self {
		Integer {
			value: self.value + other.value
		}
	}
}

impl Mul for Integer {
	type Output = Integer;

	fn mul(self, other: Integer) -> Self {
		Integer {
			value: self.value * other.value
		}
	}
}

impl Div for Integer {
	type Output = Integer;

	fn div(self, other: Integer) -> Self::Output {
		Integer {
			value: self.value / other.value
		}
	}
}


impl Display for Integer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value)
	}
}

impl From<i64> for Integer {
	fn from(x: i64) -> Self {
		Integer {
			value: BigInt::from(x)
		}
	}
}

impl NeedsParens for Integer {
	fn needs_parens(&self) -> bool {
		self.is_negative()
	}
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
	match a.cmp(b) {
		Ordering::Equal => a.clone(),
		Ordering::Greater => gcd(&(a - b), b),
		Ordering::Less => gcd(a, &(b - a))
	}
}

fn lcm(a: BigUint, b: BigUint) -> BigUint {
	let gcd = gcd(&a, &b);
	(a * b) / gcd
}