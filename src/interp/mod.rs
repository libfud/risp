//! A parent module

use self::basictype::BasicType;
use self::operator::OperatorType;
use self::read::tokenize;

pub mod basictype;
pub mod operator;
pub mod read;

#[deriving(Show)]
#[deriving(Clone)]
pub enum SExpr {
    Data(DataType),
    Cons(Box<SExpr>, Box<SExpr>),
    Nil,
}

#[deriving(Show)]
#[deriving(Clone)]
pub enum DataType {
    Operator(OperatorType),
    Literal(BasicType)
}

pub fn car(sexpr: &SExpr) -> Result<SExpr, bool> {
    match sexpr {
        &Data(ref anterior)     => Ok(Data(anterior.clone())),
        &Cons(ref anterior, _)  => Ok(*anterior.clone()),
        &Nil        => Err(false) //need to figure out how to represent nil
    }
}

pub fn cdr(sexpr: &SExpr) -> Result<SExpr, bool> {
    match sexpr {
        &Cons(_, ref dorsal)    => Ok(Cons(box *dorsal.clone(), box Nil)),
        &Nil | &Data(_)         => Err(false)
    }
}
