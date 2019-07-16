use pest::Parser;
use crate::exact::{Exact, IrrationalConstant};
use pest::iterators::{Pairs, Pair};
use num::{BigRational, BigInt};
use std::str::FromStr;
use bounds::bound::BoundType::Exclusive;

#[derive(Parser)]
#[grammar = "parser.pest"]
struct ExactParser;

#[derive(Debug)]
pub enum ParseError {
    PestError(pest::error::Error<Rule>)
}

impl From<pest::error::Error<Rule>> for ParseError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        ParseError::PestError(err)
    }
}

pub fn parse(value: &str) -> Result<Exact, ParseError> {
    let mut statement_pairs = ExactParser::parse(Rule::math, value)?;
    let statement_pair = statement_pairs.next().unwrap();
    parse_statement(statement_pair)
}

pub fn parse_statement(mut statement: Pair<Rule>) -> Result<Exact, ParseError> {
    match statement.as_rule() {
        Rule::constant => parse_constant(statement.into_inner()),
//        Rule::variable => parse_variable(statement.as_str()),
        Rule::add => {
            let mut children = statement.into_inner();
            let first = parse_statement(children.next().unwrap())?;
            let second = parse_statement(children.next().unwrap())?;
            Ok(Exact::Add(Box::new(first), Box::new(second)))
        }
        Rule::subtract => {
            let mut children = statement.into_inner();
            let first = parse_statement(children.next().unwrap())?;
            let second = parse_statement(children.next().unwrap())?;
            Ok(Exact::Sub(Box::new(first), Box::new(second)))
        }
        Rule::multiply => {
            let mut children = statement.into_inner();
            let first = parse_statement(children.next().unwrap())?;
            let second = parse_statement(children.next().unwrap())?;
            Ok(Exact::Mul(Box::new(first), Box::new(second)))
        }
        Rule::divide => {
            let mut children = statement.into_inner();
            let first = parse_statement(children.next().unwrap())?;
            let second = parse_statement(children.next().unwrap())?;
            Ok(Exact::Div(Box::new(first), Box::new(second)))
        }
        unknown => panic!("Unknown statement rule: {:?}", unknown)
    }
}

pub fn parse_constant(mut constant: Pairs<Rule>) -> Result<Exact, ParseError> {
    let constant = constant.next().unwrap();
    match constant.as_rule() {
        Rule::number_constant => parse_integer(constant),
        Rule::pi_constant => Ok(Exact::IrrationalConstant(IrrationalConstant::Pi)),
        unknown => panic!("Unknown constant type: {:?}", unknown)
    }
}

//pub fn parse_variable(mut variable_str: &str) -> Result<Exact, ParseError> {
//    Ok(Exact::Variable(variable_str.to_owned()))
//}

pub fn parse_integer(constant: Pair<Rule>) -> Result<Exact, ParseError> {
    let mut parts = constant.into_inner();
//    println!("Parts: {:?}", parts.as_str());
    let whole_part = parts.next().unwrap().as_str();
    let optional_decimal_part = parts.next().map(|x| &x.as_str()[1..]);
//    println!("Whole: {:?}", whole_part);
//    println!("decimal: {:?}", optional_decimal_part);

    let mut output = BigRational::from(BigInt::from_str(whole_part).unwrap());
    if let Some(decimal_part) = optional_decimal_part {
        let top = BigInt::from_str(decimal_part).unwrap();
        let bot = num::pow(BigInt::from(10), decimal_part.len());
        output = output + BigRational::from((top, bot));
    }
    Ok(Exact::Rational(output))
}
