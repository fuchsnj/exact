use exact::{Exact, Sqrt};

#[test]
pub fn test() {
	let x = Exact::from_u64(0);
	println!("Output = {}", x);
}

#[test]
pub fn testFormatZero() {
	let str = format!("{}", Exact::from_u64(0));
	assert_eq!(str, "0");
}

#[test]
pub fn testFormatPow() {
	let x = Exact::from_u64(2).sqrt();
	assert_eq!(format!("{}", x), "2^(1/2)");
}

#[test]
pub fn testZeroEqualsZero() {
	let zero = Exact::from_u64(0);
	assert_eq!(zero, zero);
}

#[test]
pub fn testDivision() {
	let oneHalf = Exact::from_u64(1) / Exact::from_u64(2);
	assert_eq!(format!("{}", oneHalf), "1/2");
}