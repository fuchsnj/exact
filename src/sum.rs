//use num::{BigRational, BigUint, Zero, One, BigInt};
//use std::thread;
//use bounds::Bounds;
//use sign::Sign;
//
////pub trait InfiniteSeries {
////	type Step;
////}
//
////impl IntoIterator for InfiniteSeries {
////	type Item = ();
////	type IntoIter = ();
////
////	fn into_iter(self) -> Self::IntoIter {
////		unimplemented!()
////	}
////}
//
//pub struct InfiniteSumIterator<S: InfiniteSum + ? Sized> {
//	prev_result: Option<InfiniteSumResult<S::Step>>
//}
//
//impl<S: InfiniteSum + ? Sized> Iterator for InfiniteSumIterator<S> {
//	type Item = Bounds<BigRational>;
//
//	fn next(&mut self) -> Option<Bounds<BigRational>> {
//		match self.prev_result.take() {
//			Some(result) => {
//				let result = S::next(result);
//				self.prev_result = Some(result.clone());
//				Some(result.result)
//			}
//			None => {
//				let result = S::first();
//				self.prev_result = Some(result.clone());
//				Some(result.result)
//			}
//		}
//	}
//}
//
//
//pub trait InfiniteSum {
//	type Step: Clone;
//
//	fn first() -> InfiniteSumResult<Self::Step>;
//
//	fn next(prev_result: InfiniteSumResult<Self::Step>) -> InfiniteSumResult<Self::Step>;
//
//	fn inf_sum() -> InfiniteSumIterator<Self> {
//		InfiniteSumIterator {
//			prev_result: None
//		}
//	}
//}
//
//#[test]
//fn test() {
//	//	let mut inf_sum = Pi::inf_sum();
//	//	for _ in 0..1000{
//	//		let sum = inf_sum.next().unwrap();
//	//		println!("sum: {}", sum);
//	//	}
//	//	panic!("test panic");
//}
//
//#[derive(Clone)]
//pub struct InfiniteSumResult<T: Clone> {
//	step: T,
//	bounds: Bounds<BigRational>
//}
//
//#[derive(Clone)]
//pub struct PiStep {
//	positive: bool,
//	denom: BigInt
//}
//
//struct AlternatingInfiniteSumResult<T>{
//	value: BigRational,
//	step: T
//}
//
//trait AlternatingInfiniteSum{
//	type Step;
//
//	fn first() -> (AlternatingInfiniteSumResult<Self::Step>, Sign);
//
//	fn next(prev: &BigRational)
//
//}
//
//impl <T: AlternatingInfiniteSum> InfiniteSum for T{
//	type Step = Self::Step;
//
//	fn first() -> InfiniteSumResult<Self::Step> {
//		unimplemented!()
//	}
//
//	fn next(prev_result: InfiniteSumResult<Self::Step>) -> InfiniteSumResult<Self::Step> {
//		unimplemented!()
//	}
//}
//
//
//pub struct Pi;
//
//impl InfiniteSum for Pi {
//	type Step = PiStep;
//
//	fn first() -> InfiniteSumResult<PiStep> {
//		InfiniteSumResult {
//			step: PiStep {
//				positive: true,
//				denom: BigInt::from(1 as u64),
//			},
//			bounds: Bounds::from(..BigRational::from(BigInt::from(4 as u64))),
//		}
//	}
//
//	fn next(prev_result: InfiniteSumResult<PiStep>) -> InfiniteSumResult<PiStep> {
//
//		let start = prev_result.bounds;
//
//		let prev_sum = prev_result.result;
//		let prev_step = prev_result.step;
//
//		let two = BigInt::from(2 as u64);
//		let four = BigInt::from(4 as u64);
//
//		let current_step = PiStep {
//			positive: !prev_step.positive,
//			denom: prev_step.denom + two,
//		};
//
//		let term = BigRational::from((four, current_step.denom.clone()));
//		let term = if current_step.positive {
//			term
//		} else {
//			-term
//		};
//
//		InfiniteSumResult {
//			step: current_step,
//			bounds: prev_sum + term,
//		}
//	}
//}
//
