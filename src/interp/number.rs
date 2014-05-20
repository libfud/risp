//! Numbers

pub extern crate num;

use self::num::{complex, rational, bigint};

pub enum NumericType {
    Complex(complex::Complex),
    BigRational(rational::BigRational),
    Rational(rational::Rational),
    BigInteger(bigint::BigInt),
    Floating(f64),
    Integer(int),
    UInteger(uint)
}
