use num::{BigRational, BigInt};

use std::ops::{Mul, Div, Neg, Add, Sub};
use std::fmt;
use std::cmp::Ordering;
use std::cmp;
use bounds::bounds::Bounds;
use bounds::comparison::Comparison as BoundsComparison;

use num;
use crate::sum::{InfiniteSum, pi};
use crate::parser::parse;
//pub struct PrecisionExact {
//	precision: BigRational,
//	exact: Exact,
//}

#[derive(Clone, Debug)]
pub enum IrrationalConstant {
    Pi
}

impl IrrationalConstant {
    /**
 * Retrieve the upper/lower bounds of 'self' with a minimum accuracy
 */
    fn bounds(&self, accuracy: &BigRational) -> Bounds<BigRational> {
        match self {
            IrrationalConstant::Pi => pi().get_bounds(accuracy)
        }
    }

    fn to_tex(&self) -> String {
        match self {
            IrrationalConstant::Pi => "\\pi".into()
        }
    }
}

impl fmt::Display for IrrationalConstant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IrrationalConstant::Pi => write!(f, "π"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
pub enum Comparison {
    Greater,
    Less,
    Equal,
    BoundsIntersect,
}

impl From<cmp::Ordering> for Comparison {
    fn from(value: Ordering) -> Self {
        match value {
            Ordering::Equal => Comparison::Equal,
            Ordering::Less => Comparison::Less,
            Ordering::Greater => Comparison::Greater
        }
    }
}

fn tex_paren(value: &str) -> String {
    format!("\\left({}\\right)", value)
}

fn tex_group(value: &str) -> String {
    format!("{{{}}}", value)
}

#[derive(Clone, Debug)]
pub enum Exact {
    Rational(BigRational),
    IrrationalConstant(IrrationalConstant),
    Neg(Box<Exact>),
    Add(Box<Exact>, Box<Exact>),
    Mul(Box<Exact>, Box<Exact>),
    Sub(Box<Exact>, Box<Exact>),
    Div(Box<Exact>, Box<Exact>),
    Sqrt(Box<Exact>),
}

impl Exact {
    pub fn sqrt(&self) -> Exact {
        Exact::Sqrt(Box::new(self.clone()))
    }

    pub fn to_tex(&self) -> String {
        match self {
            Exact::Rational(x) => format!("{}", x),
            Exact::IrrationalConstant(x) => x.to_tex(),
            Exact::Mul(x, y) => format!("{}*{}", tex_paren(&x.to_tex()), tex_paren(&y.to_tex())),
            Exact::Div(x, y) => format!("{}\\over{}", tex_group(&x.to_tex()), tex_group(&y.to_tex())),
            Exact::Add(x, y) => format!("{}+{}", tex_paren(&x.to_tex()), tex_paren(&y.to_tex())),
            Exact::Sub(x, y) => format!("{}-{}", tex_paren(&x.to_tex()), tex_paren(&y.to_tex())),
            Exact::Neg(x) => format!("-{}", tex_paren(&x.to_tex())),
            Exact::Sqrt(x) => format!("\\sqrt{}", tex_group(&x.to_tex()))
        }
    }

    /**
     * Retrieves an upper/lower bounds close to the request accuracy
     */
    fn bounds_accuracy_estimate(&self, accuracy_estimate: &BigRational) -> Option<Bounds<BigRational>> {
        match self {
            Exact::Rational(x) => Some(Bounds::Exact(x.clone())),
            Exact::IrrationalConstant(x) => Some(x.bounds(accuracy_estimate)),
            Exact::Neg(x) => Some(-x.bounds(accuracy_estimate)),
            Exact::Add(a, b) => Some(a.bounds(accuracy_estimate) + b.bounds(accuracy_estimate)),
            Exact::Sub(a, b) => Some(a.bounds(accuracy_estimate) - b.bounds(accuracy_estimate)),
            Exact::Mul(ref a, ref b) => {
                let ax: Option<Bounds<BigRational>> = a.bounds_accuracy_estimate(accuracy_estimate);
                let bx: Option<Bounds<BigRational>> = b.bounds_accuracy_estimate(accuracy_estimate);
                match (ax, bx) {
                    (Some(ax), Some(bx)) => ax * bx,
                    _ => None
                }
            }
            Exact::Div(ref _a, ref _b) => {
                unimplemented!()
            }
            Exact::Sqrt(ref a) => unimplemented!()
        }
        //        match *self {
//            Exact::Rational(ref x) => Bounds::Exact(x.clone()),
//            Exact::Pi => {
//                sum::pi().get_bounds(accuracy)
//            }
//            Exact::Neg(ref x) => -x.bounds(accuracy),
//            Exact::Add(ref a, ref b) => a.bounds(accuracy) + b.bounds(accuracy),
//            Exact::Sub(ref a, ref b) => a.bounds(accuracy) - b.bounds(accuracy),
//            Exact::Mul(ref a, ref b) => {
//                unimplemented!()
//            }
//            Exact::Div(ref a, ref b) => {
//                unimplemented!()
//            }
//        }
    }

    /**
     * Retrieve the upper/lower bounds of 'self' with a minimum accuracy
     */
    fn bounds(&self, accuracy: &BigRational) -> Bounds<BigRational> {
        let mut accuracy = accuracy.clone();
        loop {
            let bounds = self.bounds_accuracy_estimate(&accuracy);
            if let Some(bounds) = bounds {
                if let Some(bounds_size) = bounds.size() {
                    if bounds_size <= accuracy {
                        return bounds;
                    }
                }
            } else {
                //TODO: determine if this can actually be None
                panic!("This is a bug: Unexpected N/A bounds");
            }
            accuracy = accuracy / BigRational::from(BigInt::from(2));
        }
    }


    pub fn to_f64_inaccurate(&self) -> f64 {
        //TODO: make this take a precision argument (or ensure it's more accurate than an f64)
        // This is just a temporary function that is not considered accurate
//        match self {
//            Exact::Rational(a) => a.
//        }
        unimplemented!()
    }

//    pub fn compare_to_symbolic(&self, _other: &Self) -> Option<cmp::Ordering> {
//        None
//    }
//
//    pub fn compare_to(&self, other: &Exact, precision: &BigRational) -> Comparison {
//        self.compare_to_symbolic(other)
//            .map(|x|x.into())
//            .unwrap_or_else(|| self.compare_to_precision(other, precision))
//    }
//    pub fn compare_to_precision(&self, other: &Exact, precision: &BigRational) -> Comparison {
//        let half_precision = precision / BigRational::from_integer(BigInt::from(2));
//
//        let a_bounds = self.bounds(&half_precision);
//        let other_precision = match a_bounds.size() {
//            Some(size) => precision - size,
//            None => half_precision
//        };
//
//        let b_bounds = other.bounds(&other_precision);
//        match a_bounds.compare_to(&b_bounds) {
//            BoundsComparison::Greater => Comparison::Greater,
//            BoundsComparison::Less => Comparison::Less,
//            BoundsComparison::Intersects => Comparison::BoundsIntersect
//        }
//    }
//
//
//    pub fn equals(&self, other: &Exact, precision: &BigRational) -> bool {
//        let comparison = self.compare_to(other, precision);
//        comparison == Comparison::Equal ||
//                comparison == Comparison::BoundsIntersect
//    }
}

impl fmt::Display for Exact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Exact::Rational(ref x) => write!(f, "{}", x),
            Exact::IrrationalConstant(ref x) => write!(f, "{}", x),
            Exact::Neg(ref x) => write!(f, "-({})", x),
            Exact::Add(ref a, ref b) => write!(f, "({})+({})", a, b),
            Exact::Sub(ref a, ref b) => write!(f, "({})-({})", a, b),
            Exact::Mul(ref a, ref b) => write!(f, "({})*({})", a, b),
            Exact::Div(ref a, ref b) => write!(f, "({})/({})", a, b),
            Exact::Sqrt(ref a) => write!(f, "sqrt({})", a)
        }
    }
}

impl Neg for Exact {
    type Output = Exact;

    fn neg(self) -> Exact {
        match self {
            Exact::Rational(x) => Exact::Rational(-x),
            x => Exact::Neg(Box::new(x))
        }
    }
}

impl Add for Exact {
    type Output = Exact;

    fn add(self, other: Exact) -> Exact {
        match (self, other) {
            (Exact::Rational(a), Exact::Rational(b)) => Exact::Rational(a + b),
            (a, b) => Exact::Add(Box::new(a), Box::new(b))
        }
    }
}

impl Mul for Exact {
    type Output = Exact;

    fn mul(self, other: Exact) -> Exact {
        match (self, other) {
            (Exact::Rational(a), Exact::Rational(b)) => Exact::Rational(a * b),
            (a, b) => Exact::Mul(Box::new(a), Box::new(b))
        }
    }
}

impl Sub for Exact {
    type Output = Exact;

    fn sub(self, other: Exact) -> Exact {
        match (self, other) {
            (Exact::Rational(a), Exact::Rational(b)) => Exact::Rational(a - b),
            (a, b) => Exact::Sub(Box::new(a), Box::new(b))
        }
    }
}

impl Div for Exact {
    type Output = Exact;

    fn div(self, other: Exact) -> Exact {
        match (self, other) {
            (Exact::Rational(a), Exact::Rational(b)) => Exact::Rational(a / b),
            (a, b) => Exact::Div(Box::new(a), Box::new(b))
        }
    }
}

impl From<i64> for Exact {
    fn from(value: i64) -> Self {
        Exact::Rational(BigRational::from_integer(BigInt::from(value)))
    }
}

impl<'a> From<&'a str> for Exact {
    fn from(str: &'a str) -> Self {
        parse(str).unwrap()
    }
}


//struct StringParser;
//
//impl StringParser {
//    pub fn parse(str: &str) -> Exact {//TODO: return result and remove unwraps
//        Self::parser().parse(&mut pom::TextInput::new(str)).unwrap()
//    }
//
//    fn integer() -> Parser<char, Exact> {
//        parser::one_of("0123456789").repeat(1..)
//            .collect()
//            .map(String::from_iter)
//            .map(|str| {
//                let int = BigInt::from_str(str.as_str()).unwrap();
//                Exact::Rational(BigRational::from_integer(int))
//            })
//    }
//
//    fn decimal() -> Parser<char, Exact> {
//        parser::sym('.') * (parser::one_of("0123456789").repeat(1..)
//            .collect()
//            .map(String::from_iter)
//            .map(|str| {
//                let top = BigInt::from_str(str.as_str()).unwrap();
//                let bot = num::pow(BigInt::from(10), str.len());
//                Exact::Rational(BigRational::from((top, bot)))
//            }))
//    }
//
//    fn number() -> Parser<char, Exact> {
//        (Self::integer() + Self::decimal().opt())
//            .map(|(base, dec)| {
//                if let Some(dec) = dec {
//                    base + dec
//                } else {
//                    base
//                }
//            })
//    }
//    fn paren() -> Parser<char, Exact> {
//        parser::sym('(') * Self::expression() - parser::sym(')')
//    }
//
//    fn parser() -> Parser<char, Exact> {
//        Self::expression() - parser::end()
//    }
//
//    fn expression() -> Parser<char, Exact> {
//        Self::add()
//    }
//
//    fn add() -> Parser<char, Exact> {
//        (Self::multiply() + (parser::one_of("+-") + parser::call(Self::add)).repeat(0..))
//            .map(|(a, o)| o.iter().fold(a, |b, &(t, ref c)| {
//                if t == '+' {
//                    b + c.clone()
//                } else {
//                    b - c.clone()
//                }
//            }))
//    }
//
//    fn multiply() -> Parser<char, Exact> {
//        (Self::negate() + (parser::one_of("*/") + parser::call(Self::multiply)).repeat(0..))
//            .map(|(a, o)| o.iter().fold(a, |b, &(t, ref c)| {
//                if t == '*' {
//                    b * c.clone()
//                } else {
//                    b / c.clone()
//                }
//            }))
//    }
//
//    fn negate() -> Parser<char, Exact> {
//        (
//            parser::sym('-') * Self::atom()
//                .map(|x| Exact::neg(x))
//        ) | Self::atom()
//    }
//
//    fn pi() -> Parser<char, Exact> {
//        (parser::seq("pi") | parser::seq("π"))
//            .map(|_| Exact::IrrationalConstant(IrrationalConstant::Pi))
//    }
//
//    fn atom() -> Parser<char, Exact> {
//        Self::number() | Self::pi() | parser::call(Self::paren)
//    }
//}