use exact::{Exact, NeedsParens};
use std::fmt::{self, Display};
use std::ops::{Div, Mul, Add};
use natural::Natural;
use signed::Signed;
use num::Zero;


#[derive(Clone)]
pub struct Fraction {
	top: Natural,
	bot: Natural
}

impl Fraction {
	pub fn recip(self) -> Fraction {
		Fraction {
			top: self.bot,
			bot: self.top
		}
	}
}

impl Display for Fraction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}/{}", self.top, self.bot)
	}
}

impl Zero for Fraction {
	fn zero() -> Self {
		//TODO: is this needed?
		Fraction {
			top: Natural::zero(),
			bot: Natural::from(1),
		}
	}

	fn is_zero(&self) -> bool {
		self.top.is_zero() && !self.bot.is_zero()
	}
}

impl Add for Fraction {
	type Output = Self;

	fn add(self, _other: Fraction) -> Self::Output {
		unimplemented!()//TODO: implement
	}
}

impl NeedsParens for Fraction {
	fn needs_parens(&self) -> bool {
		true
	}
}

impl From<Natural> for Fraction {
	fn from(x: Natural) -> Self {
		Fraction {
			top: x,
			bot: 1.into()
		}
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
		if bot == 1.into() {
			Exact::Integer(Signed::positive(top.clone()))
		} else {
			Exact::Fraction(Signed::positive(Fraction {
				top,
				bot
			}))
		}
	}
}


impl From<Signed<Natural>> for Signed<Fraction> {
	fn from(natural: Signed<Natural>) -> Self {
		Signed {
			sign: natural.sign,
			value: natural.value.into(),
		}
	}
}

