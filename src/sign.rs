use std::ops::Neg;

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