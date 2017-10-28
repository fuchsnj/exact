use std::fmt;
use exact::NeedsParens;
use std::ops::{Neg, Div, Mul, Add, Sub};
use num::Zero;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Sign {
	Positive,
	Negative
}

impl Neg for Sign {
	type Output = Sign;

	fn neg(self) -> Sign {
		match self {
			Sign::Positive => Sign::Negative,
			Sign::Negative => Sign::Positive
		}
	}
}

impl Mul for Sign {
	type Output = Sign;

	fn mul(self, other: Sign) -> Sign {
		match self {
			Sign::Positive => other,
			Sign::Negative => -other
		}
	}
}

pub struct Signed<T> {
	pub sign: Sign,
	pub value: T
}

impl<T> Signed<T> {
	pub fn new(sign: Sign, value: T) -> Signed<T> {
		Signed { sign, value }
	}
	pub fn positive(value: T) -> Signed<T> {
		Signed::new(Sign::Positive, value)
	}
}

impl<T> Signed<T> {
	pub fn abs(self) -> T {
		self.value
	}
}

impl<T: Zero> Signed<T> {
	pub fn negative(value: T) -> Signed<T> {
		if value.is_zero() {
			Signed::new(Sign::Positive, value)
		} else {
			Signed::new(Sign::Negative, value)
		}
	}
}

impl<T: Clone> Clone for Signed<T> {
	fn clone(&self) -> Self {
		Signed {
			sign: self.sign,
			value: self.value.clone()
		}
	}
}

impl<T: fmt::Display + NeedsParens> fmt::Display for Signed<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.sign {
			Sign::Positive => self.value.fmt(f),
			Sign::Negative => match self.value.needs_parens() {
				true => write!(f, "-({})", self.value),
				false => write!(f, "-{}", self.value)
			}
		}
	}
}


impl<T: Zero> Neg for Signed<T> {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		if !self.value.is_zero() {
			self.sign = -self.sign
		}
		self
	}
}

impl<T> Div for Signed<T>
	where T: Div,
	      <T as Div>::Output: Mul<Sign> {
	type Output = <<T as Div>::Output as Mul<Sign>>::Output;

	fn div(self, other: Signed<T>) -> Self::Output {
		let sign = self.sign * other.sign;
		(self.value / other.value) * sign
	}
}

impl<T> Mul for Signed<T>
	where T: Mul,
	      <T as Mul>::Output: Mul<Sign> {
	type Output = <<T as Mul>::Output as Mul<Sign>>::Output;

	fn mul(self, other: Signed<T>) -> Self::Output {
		let sign = self.sign * other.sign;
		(self.value * other.value) * sign
	}
}

impl<T, O> Add for Signed<T>
	where T: Add<Output=O>,
	      T: Sub<Output=Signed<O>> {
	type Output = Signed<O>;

	fn add(self, other: Signed<T>) -> Self::Output {
		if self.sign == other.sign {
			Signed::new(self.sign, self.value + other.value)
		} else {
			if other.sign == Sign::Negative {
				self.value - other.value
			} else {
				other.value - self.value
			}
		}
	}
}

impl<T: From<u64>> From<i64> for Signed<T> {
	fn from(x: i64) -> Self {
		Signed {
			sign: if x < 0 { Sign::Negative } else { Sign::Positive },
			value: T::from(x.abs() as u64),
		}
	}
}