use num::{BigRational, BigInt};
use std::ops::{Mul, Div, Neg, Add, Sub};
use std::str::FromStr;
use pom::{self, parser, Parser};
use std::iter::FromIterator;
use std::fmt;
use std::cmp::Ordering;
use std::cmp;
use bounds::{Bounds, Comparison};

//pub struct PrecisionExact {
//	precision: BigRational,
//	exact: Exact,
//}


#[derive(Clone)]
pub enum Exact {
	Rational(BigRational),
	Pi,
	Neg(Box<Exact>),
	Add(Box<Exact>, Box<Exact>),
}

impl Exact {
//	pub fn precision(self, precision: BigRational) -> PrecisionExact {
//		PrecisionExact {
//			precision,
//			exact: self,
//		}
//	}

	pub fn bounds(&self, precision: &BigRational) -> Bounds<BigRational> {
		match *self {
			Exact::Rational(ref x) => Bounds::Exact(x.clone()),
			Exact::Pi => {
				unimplemented!()
			}
			Exact::Neg(x) => -x.bounds(precision),
			Exact::Add(a, b) => a.bounds(precision) + b.bounds(precision)
		}
	}

	pub fn compare_to(&self, other: &Exact, precision: &BigRational) -> cmp::Ordering {
		let half_precision = precision / BigRational::from_integer(BigInt::from(2));

		let a_bounds = self.bounds(&half_precision);
		let other_precision = match a_bounds.size() {
			Some(size) => precision - size,
			None => half_precision
		};

		let b_bounds = other.bounds(&other_precision);
		match a_bounds.compare_to(&b_bounds) {
			Comparison::Greater => Ordering::Greater,
			Comparison::Less => Ordering::Less,
			Comparison::Intersects => Ordering::Equal
		}
	}


	pub fn equals(&self, other: &Exact, precision: &BigRational) -> bool {
		self.compare_to(other, precision) == cmp::Ordering::Equal
	}
}

impl fmt::Display for Exact {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Exact::Rational(ref x) => write!(f, "{}", x),
			Exact::Pi => write!(f, "π"),
			Exact::Neg(x) => write!(f, "-({})", x)
		}
	}
}

impl Neg for Exact {
	type Output = Exact;

	fn neg(self) -> Exact {
		match self {
			Exact::Rational(x) => Exact::Rational(-x),
			x => Exact::Neg(Box::new(x))
		}
	}
}

impl Add for Exact {
	type Output = Exact;

	fn add(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Rational(a), Exact::Rational(b)) => Exact::Rational(a + b),
			(a, b) => Exact::Add(Box::new(a), Box::new(b))
		}
	}
}

impl Mul for Exact {
	type Output = Exact;

	fn mul(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Rational(a), Exact::Rational(b)) => {
				Exact::Rational(a * b)
			}
		}
	}
}

impl Sub for Exact {
	type Output = Exact;

	fn sub(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Rational(a), Exact::Rational(b)) => {
				Exact::Rational(a - b)
			}
		}
	}
}

impl Div for Exact {
	type Output = Exact;

	fn div(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Rational(a), Exact::Rational(b)) => {
				Exact::Rational(a / b)
			}
		}
	}
}

impl From<i64> for Exact {
	fn from(value: i64) -> Self {
		Exact::Rational(BigRational::from_integer(BigInt::from(value)))
	}
}

impl<'a> From<&'a str> for Exact {
	fn from(str: &'a str) -> Self {
		StringParser::parse(str)
	}
}


struct StringParser;

impl StringParser {
	pub fn parse(str: &str) -> Exact {//TODO: return result
		Self::parser().parse(&mut pom::TextInput::new(str)).unwrap()
	}
	fn number() -> Parser<char, Exact> {
		parser::one_of("0123456789").repeat(1..)//TODO:convert without `i64` to prevent precision loss
			.collect()
			.map(String::from_iter)
			.convert(|str| i64::from_str(&str))
			.map(Exact::from)
	}
	fn paren() -> Parser<char, Exact> {
		parser::sym('(') * Self::expression() - parser::sym(')')
	}

	fn parser() -> Parser<char, Exact> {
		Self::expression() - parser::end()
	}

	fn expression() -> Parser<char, Exact> {
		Self::add()
	}

	fn add() -> Parser<char, Exact> {
		(Self::multiply() + (parser::one_of("+-") + parser::call(Self::add)).repeat(0..))
			.map(|(a, o)| o.iter().fold(a, |b, &(t, ref c)| {
				if t == '+' {
					b + c.clone()
				} else {
					b - c.clone()
				}
			}))
	}

	fn multiply() -> Parser<char, Exact> {
		(Self::negate() + (parser::one_of("*/") + parser::call(Self::multiply)).repeat(0..))
			.map(|(a, o)| o.iter().fold(a, |b, &(t, ref c)| {
				if t == '*' {
					b * c.clone()
				} else {
					b / c.clone()
				}
			}))
	}

	fn negate() -> Parser<char, Exact> {
		(
			parser::sym('-') * Self::atom()
				.map(|x| Exact::neg(x))
		) | Self::atom()
	}

	fn atom() -> Parser<char, Exact> {
		Self::number() | parser::call(Self::paren)
	}
}