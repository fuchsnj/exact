use exact::{Exact, NeedsParens};
use integer::Integer;
use std::fmt::{self, Display};
use std::ops::{Neg, Div, Mul};

#[derive(Clone)]
pub struct Fraction {
	positive: bool,
	top: Integer,
	bot: Integer
}

impl Fraction {
	pub fn recip(self) -> Fraction {
		Fraction {
			positive: self.positive,
			top: self.bot,
			bot: self.top
		}
	}
}

impl From<Integer> for Fraction {
	fn from(x: Integer) -> Self {
		Fraction {
			positive: x.is_positive(),
			top: x.abs(),
			bot: 1.into()
		}
	}
}

impl Neg for Fraction {
	type Output = Self;

	fn neg(mut self) -> Self {
		self.positive = !self.positive;
		self
	}
}

impl Div for Fraction {
	type Output = Exact;

	fn div(self, other: Self) -> Exact {
		self * other.recip()
	}
}

impl Mul for Fraction {
	type Output = Exact;

	fn mul(self, other: Self) -> Exact {
		let mut top = self.top * other.top;
		let mut bot = self.bot * other.bot;
		let gcd = top.gcd(&bot);
		top = top / gcd.clone();
		bot = bot / gcd;
		let positive = !((!self.positive) ^ (!other.positive));

		if bot.is_one(){
			if positive{
				Exact::Integer(top.clone())
			}else{
				Exact::Integer(-top.clone())
			}
			//
		}else{
			Exact::Fraction(Fraction {
				positive: !((!self.positive) ^ (!other.positive)),
				top,
				bot
			})
		}
	}
}

impl Display for Fraction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.positive {
			write!(f, "{}/{}", self.top, self.bot)
		} else {
			write!(f, "-{}/{}", self.top, self.bot)
		}
	}
}

impl NeedsParens for Fraction {
	fn needs_parens(&self) -> bool {
		true
	}
}