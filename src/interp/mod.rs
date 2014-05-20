//! A parent module

use self::basictype::BasicType;
use self::operator::OperatorType;

pub mod basictype;
pub mod operator;

#[deriving(Show)]
#[deriving(Clone)]
pub enum SExpr {
    Cons(DataType, Box<SExpr>),
    Nil,
}

#[deriving(Show)]
#[deriving(Clone)]
pub enum DataType {
    Operator(OperatorType),
    Literal(BasicType)
}

pub fn car(sexpr: &SExpr) -> Result<DataType, bool> {
    match sexpr {
        &Cons(ref x, _) => Ok(x.clone()),
        &Nil        => Err(false) //need to figure out how to represent nil
    }
}

pub fn cdr(sexpr: &SExpr) -> Result<SExpr, bool> {
    match sexpr {
        &Cons(_, box ref dorsal) => Ok(dorsal.clone()),
        &Nil    => Err(false)
    }
}
