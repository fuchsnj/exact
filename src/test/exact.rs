use exact::{Exact, Sqrt};

#[test]
pub fn testCases() {
	let tests = vec![
		("0", "0"),
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
		("(-30)*(-30)", "900")
	];
	for (test, expected) in tests {
		let actual = format!("{}", Exact::from_string(test));
		if actual != expected {
			panic!("\nTest failed: {}\n   Expected: {}\n   Actual: {}", test, expected, actual);
		}
	}
}