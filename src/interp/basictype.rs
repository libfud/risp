//! Basic types

extern crate num;

use self::num::{rational, bigint};
// use self::num::complex;

#[deriving(Show)]
#[deriving(Clone)]
pub enum NumericType {
//    Complex(complex::Complex),
    BigRational(rational::BigRational),
    Rational(rational::Rational),
    BigInteger(bigint::BigInt),
    Floating(f64),
    Integer(int),
    UInteger(uint)
}

#[deriving(Show)]
#[deriving(Clone)]
pub enum BasicType {
    Boolean(bool),
    Character(char),
    Number(NumericType),
    String(StrBuf),
    Symbol,
    Pair,
    List,
    Procedure
}
