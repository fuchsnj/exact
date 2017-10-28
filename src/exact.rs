use std::ops::{Mul, Sub, Neg, Div, Add};
use num::{Zero, One};
use std::f64;
use std::fmt;
use pom::parser;
use pom;
use pom::Parser;
use std::str::FromStr;
use std::iter::FromIterator;
use fraction::Fraction;
use signed::{Sign, Signed};
use natural::Natural;

pub trait NeedsParens: fmt::Display {
	fn needs_parens(&self) -> bool;
}

impl NeedsParens {
	pub fn fmt_with_parens(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.needs_parens() {
			write!(f, "({})", self)
		} else {
			write!(f, "{}", self)
		}
	}
}


pub trait Sqrt {
	type Output;

	fn sqrt(self) -> Self::Output;
}


#[derive(Clone)]
pub enum Exact {
	Integer(Signed<Natural>),
	Fraction(Signed<Fraction>),
	//	Pow(Box<Exact>, Box<Exact>),
	//	Add(Vec<Exact>),
	//	Mul(Vec<Exact>)
}

impl fmt::Display for Exact {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Exact::Integer(ref x) => x.fmt(f),
			Exact::Fraction(ref x) => x.fmt(f)
		}
	}
}

impl From<i64> for Exact {
	fn from(x: i64) -> Self {
		Exact::Integer(Signed::<Natural>::from(x))
	}
}

impl Exact {
	pub fn to_f64(&self) -> f64 {
		unimplemented!()
	}

	pub fn from_string(str: &str) -> Exact {
		StringParser::parse(str)
	}
}

impl Sqrt for Exact {
	type Output = Exact;

	fn sqrt(self) -> Exact {
		unimplemented!()
	}
}

impl Neg for Exact {
	type Output = Exact;

	fn neg(self) -> Exact {
		match self {
			Exact::Integer(x) => {
				Exact::Integer(-x)
			}
			Exact::Fraction(frac) => {
				Exact::Fraction(-frac)
			}
		}
	}
}


impl Div for Exact {
	type Output = Exact;

	fn div(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Integer(a), b) => {
				Exact::Fraction(a.into()) / b
			}
			(a, Exact::Integer(b)) => {
				a / Exact::Fraction(b.into())
			}
			(Exact::Fraction(a), Exact::Fraction(b)) => {
				a / b
			}
		}
	}
}

impl Mul for Exact {
	type Output = Exact;

	fn mul(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Integer(a), Exact::Integer(b)) => {
				Exact::Integer(a * b)
			}
			(Exact::Integer(a), b) => {
				Exact::Fraction(a.into()) * b
			}
			(a, Exact::Integer(b)) => {
				a * Exact::Fraction(b.into())
			}
			(Exact::Fraction(a), Exact::Fraction(b)) => {
				a * b
			}
		}
	}
}

impl Add for Exact {
	type Output = Exact;

	fn add(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Integer(a), Exact::Integer(b)) => {
				Exact::Integer(a + b)
			}
			_ => unimplemented!()
		}
	}
}

impl Sub for Exact {
	type Output = Exact;

	fn sub(self, other: Exact) -> Exact {
		self + (-other)
	}
}

impl One for Exact {
	fn one() -> Self {
		Exact::from(1)
	}
}

impl Zero for Exact {
	fn zero() -> Self {
		Exact::from(0)
	}

	fn is_zero(&self) -> bool {
		unimplemented!()
	}
}

impl Mul<Sign> for Exact {
	type Output = Exact;

	fn mul(self, sign: Sign) -> Exact {
		match sign {
			Sign::Positive => self,
			Sign::Negative => -self
		}
	}
}

struct StringParser;

impl StringParser {
	pub fn parse(str: &str) -> Exact {
		Self::parser().parse(&mut pom::TextInput::new(str)).unwrap()
	}
	fn number() -> Parser<char, Exact> {
		parser::one_of("01234567894").repeat(1..)//TODO:convert without `i64` to prevent precision loss
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