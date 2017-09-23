use std::ops::{Mul, Sub, Neg, Div, Add};
use std::cmp::PartialEq;
use num::{BigUint, ToPrimitive, Zero};
use std::f64;
use std::fmt;

pub trait Sqrt {
	type Output;

	fn sqrt(self) -> Self::Output;
}

#[derive(Clone, Debug, Eq)]
pub enum Exact {
	Natural(BigUint),
	Negate(Box<Exact>),
	Fraction(Box<Exact>, Box<Exact>),
	Pow(Box<Exact>, Box<Exact>),
	Add(Vec<Exact>),
	Mul(Vec<Exact>)
}

impl fmt::Display for Exact {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fn needs_paren(x: &Exact) -> bool {
			match *x{
				Exact::Natural(_) => false,
				_ => true
			}
		}
		fn paren(x: &Exact) -> String {
			if needs_paren(x) {
				format!("({})", x)
			} else {
				format!("{}", x)
			}
		}

		match *self {
			Exact::Natural(ref x) => {
				write!(f, "{}", x)
			}
			Exact::Negate(ref x) => {
				write!(f, "-{}", paren(x))
			}
			Exact::Fraction(ref x, ref y) => {
				write!(f, "{}/{}", paren(x), paren(y))
			}
			Exact::Pow(ref x, ref y) => {
				write!(f, "{}^{}", paren(x), paren(y))
			}
			Exact::Add(ref list) => {
				let mut out = String::new();
				for x in list {
					if out != "" {
						out += "+";
					}
					out += &paren(x);
				}
				write!(f, "{}", out)
			}
			Exact::Mul(ref list) => {
				let mut out = String::new();
				for x in list {
					if out != "" {
						out += "*";
					}
					out += &paren(x);
				}
				write!(f, "{}", out)
			}
		}
	}
}

impl Exact {
	pub fn fraction(x: &Exact, y: &Exact) -> Exact {
		Exact::Fraction(Box::new(x.clone()), Box::new(y.clone()))
	}

	pub fn from_u64(x: u64) -> Exact {
		Exact::Natural(BigUint::from(x))
	}

	pub fn to_f64(&self) -> f64 {
		match *self {
			Exact::Natural(ref x) => {
				x.to_f64().unwrap_or(f64::NAN)
			}
			Exact::Negate(ref x) => {
				-x.to_f64()
			}
			Exact::Fraction(ref x, ref y) => {
				x.to_f64() / y.to_f64()
			}
			Exact::Pow(ref x, ref y) => {
				x.to_f64().powf(y.to_f64())
			}
			Exact::Add(ref list) => {
				list.iter().fold(0.0, |x, y| x + y.to_f64())
			}
			Exact::Mul(ref list) => {
				list.iter().fold(1.0, |x, y| x * y.to_f64())
			}
		}
	}
	pub fn reciprocal(&self) -> Exact {
		Exact::fraction(&Exact::from_u64(1), &self.clone())
	}
}

impl Sqrt for Exact {
	type Output = Exact;

	fn sqrt(self) -> Exact {
		Exact::Pow(Box::new(self), Box::new(Exact::from_u64(1) / Exact::from_u64(2)))
	}
}

impl Neg for Exact {
	type Output = Exact;

	fn neg(self) -> Exact {
		Exact::Negate(Box::new(self))
	}
}

impl PartialEq for Exact {
	fn eq(&self, other: &Exact) -> bool {
		match (self, other) {
			(&Exact::Natural(ref x), &Exact::Natural(ref y)) => { x == y }
			(_, _) => unimplemented!()
		}
	}
}

impl Div for Exact {
	type Output = Exact;

	fn div(self, other: Exact) -> Exact {
		Exact::fraction(&self, &other)
	}
}

impl Mul for Exact {
	type Output = Exact;

	fn mul(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Natural(a), Exact::Natural(b)) => Exact::Natural(a * b),
			(x, y) => Exact::Mul(vec![x, y])
		}
	}
}

impl Add for Exact {
	type Output = Exact;

	fn add(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Natural(a), Exact::Natural(b)) => Exact::Natural(a + b),
			(x, y) => Exact::Add(vec![x, y])
		}
	}
}

impl Sub for Exact {
	type Output = Exact;

	fn sub(self, other: Exact) -> Exact {
		self + -other
	}
}