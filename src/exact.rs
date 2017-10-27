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
use integer::Integer;
use fraction::Fraction;

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
	Integer(Integer),
	Fraction(Fraction),
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
		Exact::Integer(Integer::from(x))
	}
}

impl Exact {
	pub fn to_f64(&self) -> f64 {
		unimplemented!()
	}
	//	pub fn reciprocal(&self) -> Exact {
	//		Exact::fraction(&Exact::from_u64(1), &self.clone())
	//	}

	pub fn from_string(str: &str) -> Exact {
		StringParser::parse(str)
	}

	//	fn simplify(&self) -> Exact {
	//		match *self {
	//			Exact::Integer(ref x) => { self.clone() }
	//			//			Exact::Negate(ref x) => {
	//			//				match *x.as_ref() {
	//			//					Exact::Negate(ref y) => { y.as_ref().clone() }
	//			//					Exact::Natural(_)
	//			//					| Exact::Fraction(_, _)
	//			//					| Exact::Pow(_, _)
	//			//					| Exact::Add(_)
	//			//					| Exact::Mul(_) => {
	//			//						self.clone()
	//			//					}
	//			//				}
	//			//			}
	//			//			Exact::Fraction(ref x, ref y) => {
	//			//				let mut neg_count = 0;
	//			//				let x = if let Exact::Negate(ref inner) = *x.as_ref() {
	//			//					neg_count += 1;
	//			//					inner
	//			//				} else {
	//			//					x
	//			//				};
	//			//				let y = if let Exact::Negate(ref inner) = *y.as_ref() {
	//			//					neg_count += 1;
	//			//					inner
	//			//				} else {
	//			//					y
	//			//				};
	//			//				let mut frac = Exact::Fraction(x.clone(), y.clone());
	//			//				if neg_count == 1 {
	//			//					frac = Exact::Negate(Box::new(frac));
	//			//				};
	//			//				frac
	//			//			}
	//			//			Exact::Mul(ref list) => {
	//			//				let list = list.clone();
	//			//				{
	//			//					let mut natural = BigUint::one();
	//			//					let mut natural_positive = true;
	//			//					let mut unknown_list = vec!();
	//			//					for x in list {
	//			//						match x {
	//			//							Exact::Natural(y) => {
	//			//								natural *= y;
	//			//							}
	//			//							Exact::Negate(box_y) => {
	//			//								mat
	//			//							}
	//			//							_ => unknown_list.push(x);
	//			//						}
	//			//					}
	//			//				}
	//			//
	//			//				let mut positive = true;
	//			//				let list = list.iter().map(|x| {
	//			//					if let Exact::Negate(ref boxed) = *x {
	//			//						positive = !positive;
	//			//						boxed.as_ref().clone()
	//			//					} else {
	//			//						x.clone()
	//			//					}
	//			//				}).collect();
	//			//
	//			//				let output = Exact::Mul(list);
	//			//
	//			//				if positive {
	//			//					output
	//			//				} else {
	//			//					Exact::Negate(Box::new(output))
	//			//				}
	//			//			}
	//			//			Exact::Add(ref list) => {
	//			//				let mut all_negative = true;
	//			//				let pos_list = list.iter().map(|x| {
	//			//					if let Exact::Negate(ref boxed) = *x {
	//			//						boxed.as_ref().clone()
	//			//					} else {
	//			//						all_negative = false;
	//			//						x.clone()
	//			//					}
	//			//				}).collect();
	//			//				let output = Exact::Add(if all_negative {
	//			//					pos_list
	//			//				} else {
	//			//					list.clone()
	//			//				});
	//			//
	//			//				if all_negative {
	//			//					Exact::Negate(Box::new(output))
	//			//				} else {
	//			//					output
	//			//				}
	//			//			}
	//			//			Exact::Pow(ref x, ref y) => {
	//			//				self.clone() //TODO: implement
	//			//			}
	//		}
	//	}
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
			//			(Exact::Integer(top), b @ Exact::Fraction(_, _)) | (b @ Exact::Fraction(_, _), Exact::Integer(top)) => {
			//				let top = Signed::positive(Exact::Integer(top));
			//				let bot = Exact::Integer(Signed::positive(BigUint::one()));
			//				Exact::Fraction(Box::new(top), Box::new(bot)) + b
			//			}
			//			(Exact::Fraction(a, b), Exact::Fraction(x, y)) => {
			//				match (*a.as_ref(), *b.as_ref(), *x.as_ref(), *y.as_ref()) {
			//					(Exact::Fraction(_, _), _, _, _) | (_, Exact::Fraction(_, _), _, _)
			//					| (_, _, Exact::Fraction(_, _), _) | (_, _, _, Exact::Fraction(_, _)) => unreachable!(),
			//					(Exact::Integer(a), Exact::Integer(b), Exact::Integer(x), Exact::Integer(y)) => unimplemented!()
			//				}
			//			}
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