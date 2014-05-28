//! Basic types

extern crate num;

use self::num::{rational, bigint};
use super::SExpr;
// use self::num::complex;

#[deriving(Show, Clone, Eq)]
pub enum NumericType {
//    Complex(complex::Complex),
    BigRational(rational::BigRational),
//    Rational(rational::Rational),
//    BigInteger(bigint::BigInt),
    Floating(f64),
    Integer(int),
//    UInteger(uint)
}

#[deriving(Show, Clone, Eq)]
pub enum BasicType {
    Boolean(bool),
    Character(char),
    Number(NumericType),
    String(StrBuf),
    Symbol(StrBuf),
    Pair(Box<BasicType>, Box<BasicType>),
    List,
    Procedure(Box<SExpr>)
}
