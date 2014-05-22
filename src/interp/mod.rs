//! A parent module

use self::basictype::BasicType;
use self::operator::OperatorType;
use self::read::tokenize;

pub mod basictype;
pub mod operator;
pub mod read;

///Boxing used to dynamically allocate memory to allow for the recursive data 
///structure. Data can be anything - an operator, a number, a string,
///anything which has a type.
#[deriving(Show)]
#[deriving(Clone)]
pub enum SExpr {
    Data(DataType),
    Cons(Box<SExpr>, Box<SExpr>),
    Nil,
}

///DataTypes are either operators or literals. I think this is kind of shaky
///right now because I don't know how functions and variables are going to
///work yet.
#[deriving(Show)]
#[deriving(Clone)]
pub enum DataType {
    Operator(OperatorType),
    Literal(BasicType)
}

///Returns the first atom found in a cell. E.g,
///car(Cons(box Data(Operator(Add))), box Data(Literal(Number(5))));
///returns +, where
///car(Cons(box Cons(box Literal(Number 7)), box Nil), box Nil)
///returns Cons(box Literal(Number 7), box Nil)
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
