use exact::Exact;
use std::cmp::Ordering;
use num::{BigRational, BigInt};

#[test]
pub fn test_cases() {
	let tests = vec![
		("0", "0"),
		("1", "1"),
		("-1", "-1"),
		("-(-1)", "1"),
		("-(-(-(-1)))", "1"),
		("1+1", "2"),
		("(-1)+(-1)", "-2"),
		("(-1)+1", "0"),
		("1+(-1)", "0"),
		("1-1", "0"),
		("2-1", "1"),
		("1-2", "-1"),
		("(-1)-(-1)", "0"),
		("(-1)-1", "-2"),
		("1-(-1)", "2"),
		("1*1", "1"),
		("(-30)*30", "-900"),
		("(-30)*(-30)", "900"),
		("1/2", "1/2"),
		("2/4", "1/2"),
		("(1/2)/3", "1/6"),
		("(1/2)/(3/4)", "2/3"),
		("1/(2/3)", "3/2"),
		("(4/2)/2", "1"),
		("(-1)/1", "-1"),
		("(-1)/(-1)", "1"),
		("1/1", "1"),
		("(1/2)*2", "1"),
		("(2/4)*(1/2)", "1/4"),
		("2*(1/5)", "2/5"),
		("(-2)*(1/5)", "-2/5"),
		("1+2*3", "7"),
		("1*2+3", "5"),
		("(-1)/2", "-1/2"),
		("-1/2", "-1/2"),
		("1/-2", "-1/2")
	];
	for (test, expected) in tests {
		let actual = format!("{}", Exact::from(test));
		if actual != expected {
			panic!("\nTest failed: {}\n   Expected: {}\n   Actual: {}", test, expected, actual);
		}
	}
}

#[test]
fn test_compare() {
	use self::Ordering::*;

	let precision: BigRational = BigRational::from((BigInt::from(1) / BigInt::from(10000000000_u64)));

	let tests = vec![
		("0", "0", Equal),
		("0", "1", Less),
		("1", "0", Greater)
	];
	for (a, b, expected) in tests {
		let a = Exact::from(a);
		let b = Exact::from(b);

		let actual = a.compare_to(&b, &precision);
		if actual != expected {
			panic!("\nTest failed: {} | {}\n   Expected: {:?}\n   Actual: {:?}", a, b, expected, actual);
		}
	}
}