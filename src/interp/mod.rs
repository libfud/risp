//! A parent module

extern crate collections;

use self::collections::HashMap;
use self::basictype::BasicType;
use self::operator::OperatorType;
use self::read::translate::parse;
pub use self::read::tokenize::TokenStream;

pub mod basictype;
pub mod operator;
pub mod read;
pub mod eval;

///Boxing used to dynamically allocate memory to allow for the recursive data 
///structure. Data can be anything - an operator, a number, a string,
///anything which has a type.
#[deriving(Show)]
#[deriving(Clone)]
#[deriving(Eq)]
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
#[deriving(Eq)]
pub enum DataType {
    Operator(OperatorType),
    Literal(BasicType),
    Variable(StrBuf)
}

///Returns the first atom found in a cell. E.g,
///car(Cons(box Data(Operator(Add))), box Data(Literal(Number(5))));
///returns +, where
///car(Cons(box Cons(box Literal(Number 7)), box Nil), box Nil)
///returns Cons(box Literal(Number 7), box Nil)
pub fn car(sexpr: &SExpr) -> Result<SExpr, bool> {
    match sexpr {
        &Data(ref anterior)         => Ok(Data(anterior.clone())),
        &Cons(ref ant, ref dors)    => Ok(Cons(ant.clone(), dors.clone())),
        &Nil        => Err(false) //need to figure out how to represent nil
    }
}

pub fn cdr(sexpr: &SExpr) -> Result<SExpr, bool> {
    match sexpr {
        &Cons(_, ref dorsal)    => Ok(Cons(box *dorsal.clone(), box Nil)),
        &Nil | &Data(_)         => Err(false)
    }
}

pub struct Environment {
    pub variables: HashMap<StrBuf, SExpr>
}

pub fn interp(sexpr: StrBuf, mut global_env: &Environment) -> StrBuf {

    let mut tokens = TokenStream{
        string_slice: sexpr,
        string_index: 0
    };

    let sexpr = match parse(&mut tokens) {
        Ok(good)    => good,
        Err(msg)    => return msg
    };

    println!("car of sexpr:\n{}\n", car(&*sexpr));
    println!("cdr of sexpr\n{}\n", cdr(&*sexpr));

    sexpr.to_str().to_strbuf()
}
