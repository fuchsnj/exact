use num::{BigUint, Zero, One};
use exact::NeedsParens;
use std::fmt;
use signed::{Signed, Sign};
use std::i64;
use std::ops::{Add, Neg, Mul, Div, Sub};
use std::cmp::Ordering;


#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Natural {
	value: BigUint
}

impl Natural {
	pub fn gcd(&self, b: &Natural) -> Natural {
		match self.cmp(b) {
			//TODO: impl Sub for &Natural
			Ordering::Equal => self.clone(),
			Ordering::Greater => (self.clone() - b.clone()).abs().gcd(b),
			Ordering::Less => self.gcd(&(b.clone() - self.clone()).abs())
		}
	}

	pub fn lcm(&self, b: &Natural) -> Natural {
		//TODO: impl Mul for &Natural
		(self.clone() * b.clone()) / self.gcd(b)
	}
}

impl NeedsParens for Natural {
	fn needs_parens(&self) -> bool {
		false
	}
}

impl fmt::Display for Natural {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.value.fmt(f)
	}
}

impl Zero for Natural {
	fn zero() -> Self {
		0.into()
	}

	fn is_zero(&self) -> bool {
		self.value.is_zero()
	}
}

impl One for Natural {
	fn one() -> Self {
		1.into()
	}
}

impl Add for Natural {
	type Output = Natural;

	fn add(self, other: Natural) -> Self {
		Natural {
			value: self.value + other.value
		}
	}
}

impl From<u64> for Natural {
	fn from(value: u64) -> Self {
		Natural {
			value: BigUint::from(value),
		}
	}
}

impl Mul for Natural {
	type Output = Natural;

	fn mul(self, other: Self) -> Natural {
		Natural {
			value: self.value * other.value,
		}
	}
}

impl Div for Natural {
	type Output = Natural;

	fn div(self, other: Natural) -> Self::Output {
		Natural {
			value: self.value / other.value
		}
	}
}

impl Mul<Sign> for Natural {
	type Output = Signed<Natural>;

	fn mul(self, sign: Sign) -> Signed<Natural> {
		Signed::new(sign, self)
	}
}

impl Sub for Natural {
	type Output = Signed<Natural>;

	fn sub(self, other: Natural) -> Signed<Natural> {
		if self < other {
			-(other - self)
		} else {
			Signed::positive(Natural {
				value: self.value - other.value,
			})
		}
	}
}