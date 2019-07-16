use num::{BigRational, BigInt};
use bounds::bounds::Bounds;
use bounds::bound::Bound;

pub fn pi() -> AlternatingInfiniteSum<PiAlternatingSeries> {
    AlternatingInfiniteSum::<PiAlternatingSeries>::new()
}

//#[test]
//pub fn test() {
////    let mut pi = PiAlternatingSeries::new();
////    for x in 0..10 {
////        println!("{}: {}", x, pi.next());
////    }
//    let low = &BigRational::from((BigInt::from(1), BigInt::from(10_u64)));
////    let high = &BigRational::from((BigInt::from(1) / BigInt::from(10000000000_u64)));
//    let mut pi = pi();
//    println!("Range: {:?}", pi.get_bounds(low));
//    panic!();
//}

pub trait InfiniteSum {
    fn get_bounds(&mut self, accuracy: &BigRational) -> Bounds<BigRational>;
}

pub struct AlternatingInfiniteSum<T> {
    series: T
}

impl<T: AlternatingInfiniteSeries> AlternatingInfiniteSum<T> {
    pub fn new() -> AlternatingInfiniteSum<T> {
        AlternatingInfiniteSum { series: T::new() }
    }
}

impl<T: AlternatingInfiniteSeries> InfiniteSum for AlternatingInfiniteSum<T> {
    fn get_bounds(&mut self, accuracy: &BigRational) -> Bounds<BigRational> {
        let a = self.series.next();
        let b = self.series.next() + a.clone();
        let mut min;
        let mut max;
        let mut max_next;
        if a < b {
            min = a;
            max = b.clone();
            max_next = false;
        } else {
            min = b.clone();
            max = a;
            max_next = true;
        };

        let mut sum = b;
        loop {
            println!("Min: {} Max: {}", min, max);
            let diff = max.clone() - min.clone();
            println!("Diff: {}", diff);
            if diff <= *accuracy { break; }
            sum = sum.clone() + self.series.next();
            if max_next {
                max = sum.clone();
            } else {
                min = sum.clone();
            }
            max_next = !max_next;
        }
        let offset = T::offset();
        Bounds::Range(
            Some(Bound::exclusive(min + offset.clone())),
            Some(Bound::exclusive(max + offset)),
        )
    }
}

pub trait AlternatingInfiniteSeries {
    fn new() -> Self;
    fn offset() -> BigRational;
    fn next(&mut self) -> BigRational;
}

pub struct PiAlternatingSeries {
    step: BigRational,
    positive: bool,
}

impl AlternatingInfiniteSeries for PiAlternatingSeries {
    fn new() -> Self {
        PiAlternatingSeries {
            step: BigRational::from_integer(BigInt::from(2)),
            positive: true,
        }
    }

    fn offset() -> BigRational {
        BigRational::from(BigInt::from(3))
    }

    fn next(&mut self) -> BigRational {
        let four = BigRational::from_integer(BigInt::from(4));
        let step = self.step.clone();
        let step2 = step.clone() + BigInt::from(1);
        let step3 = step2.clone() + BigInt::from(1);

        let mut output = four / (step * step2 * step3.clone());
        if !self.positive {
            output = -output;
        }
        self.step = step3;
        self.positive = !self.positive;
        output
    }
}
