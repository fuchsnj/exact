use std::ops::{Mul, Sub, Neg, Div, Add};
use std::cmp::{PartialEq, Ord, Ordering};
use num::{BigUint, ToPrimitive, Zero, One, BigInt};
use std::{f64, u64};
use std::fmt;
use pom::parser;
use pom;
use pom::Parser;
use std::str::FromStr;
use std::iter::FromIterator;
use std::ops::Deref;

pub trait NeedsParens {
	fn needs_parens(&self) -> bool;
}

pub trait Sqrt {
	type Output;

	fn sqrt(self) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct Signed<T> {
	inner: T,
	positive: bool
}

impl<T> Signed<T> {
	pub fn new(inner: T, positive: bool) -> Signed<T> {
		Signed { inner, positive }
	}
	pub fn positive(inner: T) -> Signed<T> {
		Signed::new(inner, true)
	}
	pub fn negative(inner: T) -> Signed<T> {
		Signed::new(inner, false)
	}
	pub fn is_positive(&self) -> bool {
		self.positive
	}
	pub fn is_negative(&self) -> bool {
		!self.positive
	}
	pub fn split(self) -> (bool, T) {
		(self.positive, self.inner)
	}
}

impl<T> Deref for Signed<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.inner
	}
}

impl<T: fmt::Display + NeedsParens> fmt::Display for Signed<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.is_positive() {
			self.inner.fmt(f)
		} else {
			if NeedsParens::needs_parens(&self.inner) {
				write!(f, "-({})", &self.inner)
			} else {
				write!(f, "-{}", &self.inner)
			}
		}
	}
}

impl NeedsParens for BigUint {
	fn needs_parens(&self) -> bool {
		false
	}
}

#[derive(Clone, Debug)]
pub enum Exact {
	Integer(Signed<BigUint>)
	//	Fraction(Box<Exact>, Box<Exact>),
	//	Pow(Box<Exact>, Box<Exact>),
	//	Add(Vec<Exact>),
	//	Mul(Vec<Exact>)
}

impl fmt::Display for Exact {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fn needs_paren(x: &Exact) -> bool {
			match *x {
				Exact::Integer(_) => false,
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
			Exact::Integer(ref x) => {
				write!(f, "{}", x)
			}
			//			Exact::Negate(ref x) => {
			//				write!(f, "-{}", paren(x))
			//			}
			//			Exact::Fraction(ref x, ref y) => {
			//				write!(f, "{}/{}", paren(x), paren(y))
			//			}
			//			Exact::Pow(ref x, ref y) => {
			//				write!(f, "{}^{}", paren(x), paren(y))
			//			}
			//			Exact::Add(ref list) => {
			//				let mut out = String::new();
			//				for x in list {
			//					if out != "" {
			//						out += "+";
			//					}
			//					out += &paren(x);
			//				}
			//				write!(f, "{}", out)
			//			}
			//			Exact::Mul(ref list) => {
			//				let mut out = String::new();
			//				for x in list {
			//					if out != "" {
			//						out += "*";
			//					}
			//					out += &paren(x);
			//				}
			//				write!(f, "{}", out)
			//			}
		}
	}
}

impl Exact {
	pub fn from_u64(x: u64) -> Exact {
		Exact::Integer(Signed::positive(BigUint::from(x)))
	}

	pub fn to_f64(&self) -> f64 {
		match *self {
			Exact::Integer(ref x) => {
				let pos_int = x.to_f64().unwrap_or(f64::NAN);
				if x.is_positive() {
					pos_int
				} else {
					-pos_int
				}
			}
			//			Exact::Negate(ref x) => {
			//				-x.to_f64()
			//			}
			//			Exact::Fraction(ref x, ref y) => {
			//				x.to_f64() / y.to_f64()
			//			}
			//			Exact::Pow(ref x, ref y) => {
			//				x.to_f64().powf(y.to_f64())
			//			}
			//			Exact::Add(ref list) => {
			//				list.iter().fold(0.0, |x, y| x + y.to_f64())
			//			}
			//			Exact::Mul(ref list) => {
			//				list.iter().fold(1.0, |x, y| x * y.to_f64())
			//			}
		}
	}
	//	pub fn reciprocal(&self) -> Exact {
	//		Exact::fraction(&Exact::from_u64(1), &self.clone())
	//	}

	pub fn from_string(str: &str) -> Exact {
		StringParser::parse(str).simplify()
	}

	fn simplify(&self) -> Exact {
		match *self {
			Exact::Integer(ref x) => { self.clone() }
			//			Exact::Negate(ref x) => {
			//				match *x.as_ref() {
			//					Exact::Negate(ref y) => { y.as_ref().clone() }
			//					Exact::Natural(_)
			//					| Exact::Fraction(_, _)
			//					| Exact::Pow(_, _)
			//					| Exact::Add(_)
			//					| Exact::Mul(_) => {
			//						self.clone()
			//					}
			//				}
			//			}
			//			Exact::Fraction(ref x, ref y) => {
			//				let mut neg_count = 0;
			//				let x = if let Exact::Negate(ref inner) = *x.as_ref() {
			//					neg_count += 1;
			//					inner
			//				} else {
			//					x
			//				};
			//				let y = if let Exact::Negate(ref inner) = *y.as_ref() {
			//					neg_count += 1;
			//					inner
			//				} else {
			//					y
			//				};
			//				let mut frac = Exact::Fraction(x.clone(), y.clone());
			//				if neg_count == 1 {
			//					frac = Exact::Negate(Box::new(frac));
			//				};
			//				frac
			//			}
			//			Exact::Mul(ref list) => {
			//				let list = list.clone();
			//				{
			//					let mut natural = BigUint::one();
			//					let mut natural_positive = true;
			//					let mut unknown_list = vec!();
			//					for x in list {
			//						match x {
			//							Exact::Natural(y) => {
			//								natural *= y;
			//							}
			//							Exact::Negate(box_y) => {
			//								mat
			//							}
			//							_ => unknown_list.push(x);
			//						}
			//					}
			//				}
			//
			//				let mut positive = true;
			//				let list = list.iter().map(|x| {
			//					if let Exact::Negate(ref boxed) = *x {
			//						positive = !positive;
			//						boxed.as_ref().clone()
			//					} else {
			//						x.clone()
			//					}
			//				}).collect();
			//
			//				let output = Exact::Mul(list);
			//
			//				if positive {
			//					output
			//				} else {
			//					Exact::Negate(Box::new(output))
			//				}
			//			}
			//			Exact::Add(ref list) => {
			//				let mut all_negative = true;
			//				let pos_list = list.iter().map(|x| {
			//					if let Exact::Negate(ref boxed) = *x {
			//						boxed.as_ref().clone()
			//					} else {
			//						all_negative = false;
			//						x.clone()
			//					}
			//				}).collect();
			//				let output = Exact::Add(if all_negative {
			//					pos_list
			//				} else {
			//					list.clone()
			//				});
			//
			//				if all_negative {
			//					Exact::Negate(Box::new(output))
			//				} else {
			//					output
			//				}
			//			}
			//			Exact::Pow(ref x, ref y) => {
			//				self.clone() //TODO: implement
			//			}
		}
	}
}

impl Sqrt for Exact {
	type Output = Exact;

	fn sqrt(self) -> Exact {
		unimplemented!()
		//		Exact::Pow(Box::new(self), Box::new(Exact::from_u64(1) / Exact::from_u64(2))).simplify()
	}
}

impl Neg for Exact {
	type Output = Exact;

	fn neg(self) -> Exact {
		match self {
			Exact::Integer(x) => {
				Exact::Integer(Signed::new(x.deref().clone(), !x.is_positive()))
			}
		}
	}
}


impl Div for Exact {
	type Output = Exact;

	fn div(self, other: Exact) -> Exact {
		unimplemented!()
		//		Exact::Fraction(Box::new(self), Box::new(other)).simplify()
	}
}

impl Mul for Exact {
	type Output = Exact;

	fn mul(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Integer(a), Exact::Integer(b)) => {
				let is_negative = a.is_negative() ^ b.is_negative();
				Exact::Integer(Signed::new(a.deref() * b.deref(), !is_negative))
			}
		}
	}
}

impl Add for Exact {
	type Output = Exact;

	fn add(self, other: Exact) -> Exact {
		match (self, other) {
			(Exact::Integer(a), Exact::Integer(b)) => {
				if a.is_positive() == b.is_positive() {
					Exact::Integer(Signed::new(a.deref() + b.deref(), a.is_positive()))
				} else {
					match a.deref().cmp(b.deref()) {
						Ordering::Greater => Exact::Integer(Signed::new(a.deref() - b.deref(), a.is_positive())),
						Ordering::Less => Exact::Integer(Signed::new(b.deref() - a.deref(), b.is_positive())),
						Ordering::Equal => Exact::zero()
					}
				}
			}
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
		Exact::from_u64(1)
	}
}

impl Zero for Exact {
	fn zero() -> Self {
		Exact::from_u64(0)
	}

	fn is_zero(&self) -> bool {
		unimplemented!()
	}
}

impl PartialEq for Exact {
	fn eq(&self, other: &Exact) -> bool {
		unimplemented!()
		//		match (self, other) {
		//			(&Exact::Natural(ref x), &Exact::Natural(ref y)) => { x == y }
		//			(_, _) => unimplemented!()
		//		}
	}
}


struct StringParser;

impl StringParser {
	pub fn parse(str: &str) -> Exact {
		Self::parser().parse(&mut pom::TextInput::new(str)).unwrap()
	}
	fn number() -> Parser<char, Exact> {
		parser::one_of("01234567894").repeat(1..)
				.collect()
				.map(String::from_iter)
				.convert(|str| u64::from_str(&str))
				.map(Exact::from_u64)
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