use crate::exact::Exact;
use num::{BigInt, BigRational};

#[test]
pub fn test_cases_str_to_exact() {
    let tests = vec![
        ("0", "0"),
        ("1", "1"),
        ("-1", "-1"),
//        ("-(-1)", "1"),
//        ("-(-(-(-1)))", "1"),
        ("1+1", "(1)+(1)"),
        ("(-1)+(-1)", "(-1)+(-1)"),
        ("(-1)+1", "(-1)+(1)"),
        ("1+(-1)", "(1)+(-1)"),
        ("1-1", "(1)-(1)"),
        ("2-1", "(2)-(1)"),
//        ("1-2", "-1"),
        ("(-1)-(-1)", "(-1)-(-1)"),
//        ("(-1)-1", "-2"),
//        ("1-(-1)", "2"),
        ("1*1", "(1)*(1)"),
//        ("(-30)*30", "-900"),
//        ("(-30)*(-30)", "900"),
        ("1/2", "(1)/(2)"),
        ("2/4", "(2)/(4)"),
//        ("(1/2)/3", "1/6"),
//        ("(1/2)/(3/4)", "2/3"),
//        ("1/(2/3)", "3/2"),
//        ("(4/2)/2", "1"),
//        ("(-1)/1", "-1"),
//        ("(-1)/(-1)", "1"),
//        ("1/1", "1"),
//        ("(1/2)*2", "1"),
//        ("(2/4)*(1/2)", "1/4"),
//        ("2*(1/5)", "2/5"),
//        ("(-2)*(1/5)", "-2/5"),
//        ("1+2*3", "7"),
//        ("1*2+3", "5"),
//        ("(-1)/2", "-1/2"),
//        ("-1/2", "-1/2"),
//        ("1/-2", "-1/2"),
        ("1.5", "3/2"),
        ("0.01", "1/100"),
        ("pi", "π"),
        ("π", "π"),
        ("1+π", "(1)+(π)"),
        ("2*π", "(2)*(π)"),
//        ("1+x", "(1)+(x)")
    ];
    for (test, expected) in tests {
        let actual = format!("{}", Exact::from(test));
        if actual != expected {
            panic!("\nTest failed: {}\n   Expected: {}\n   Actual: {}", test, expected, actual);
        }
    }
}

//#[test]
//fn test_compare() {
//    use crate::exact::Comparison::*;
//
//    let low = &BigRational::from((BigInt::from(1), BigInt::from(100)));
//    let high = &BigRational::from((BigInt::from(1), BigInt::from(10000000000_u64)));
//
//
//    let tests = vec![
//        ("0", "0", BoundsIntersect, high),
//        ("0", "1", Less, high),
//        ("1", "0", Greater, high),
////        ("3.1415", "3.14", Greater, low),
////        ("2*1.2", "2.4", BoundsIntersect, high),
////        ("π", "3.1", Greater, low),
////        ("π", "3.3", Less, low),
////        ("π", "pi", BoundsIntersect, low),
////        ("π", "pi", BoundsIntersect, high)
//    ];
//    for (a, b, expected, precision) in tests {
//        let a = Exact::from(a);
//        let b = Exact::from(b);
//
//        let actual = a.compare_to(&b, precision);
//        if actual != expected {
//            panic!("\nTest failed: {} | {}\n   Expected: {:?}\n   Actual: {:?}", a, b, expected, actual);
//        }
//    }
//}