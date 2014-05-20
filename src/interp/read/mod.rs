//! A parent module

use self::basictype::BasicType;
use self::operator::OperatorType;

pub mod basictype;
pub mod operator;

#[deriving(Show)]
pub enum SExpr {
    Cons(DataType, Box<SExpr>),
    Nil,
}

pub enum DataType {
    Operator(OperatorType),
    Literal(BasicType)
}
