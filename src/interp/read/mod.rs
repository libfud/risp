//! A parent module

use super::basictype::BasicType;
use super::operator::OperatorType;

pub mod tokenize;
pub mod translate;

#[deriving(Show)]
pub enum SExpr {
    Cons(DataType, Box<SExpr>),
    Nil,
}

#[deriving(Show)]
pub enum DataType {
    Operator(OperatorType),
    Literal(BasicType)
}
