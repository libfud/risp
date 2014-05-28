//! A parent module

extern crate collections;

use self::collections::HashMap;
use self::basictype::BasicType;
use self::operator::OperatorType;
use self::read::translate::parse;
use self::read::tokenize::TokenStream;
use self::eval::eval;

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
///returns +, where car(Cons(box Cons(box Literal(Number 7)), box Nil), box Nil)
///returns Cons(box Literal(Number 7), box Nil)
pub fn car(sexpr: &SExpr) -> Result<SExpr, bool> {
    match sexpr {
        &Data(ref anterior)     => Ok(Data(anterior.clone())),
        &Cons(ref anterior, _)  => Ok(*anterior.clone()),
        &Nil        => Err(false) //need to figure out how to represent nil
    }
}

///Returns the dorsal region of a cell, or Errs if the SExpr is an atom.
pub fn cdr(sexpr: &SExpr) -> Result<SExpr, bool> {
    match sexpr {
        &Cons(_, ref dorsal)    => Ok(*dorsal.clone()),
        &Nil | &Data(_)         => Err(false)
    }
}

///A representation of a frame as a HashMap of SExprs which can either be
///data in general or SExprs.
pub struct Environment {
    pub variables: HashMap<StrBuf, SExpr>,
    pub parent: Option<Box<Environment>>
}

///Returns the value of a variable, whether it's a number, string or a procedure,
///or returns an Error if no such variable is found. It recurses through
///each parent environment until it reaches the global one.
pub fn lookup(var: &str, env: &Environment) -> Result<SExpr, StrBuf> {
    match env.variables.find(&var.to_strbuf()) {
        Some(val)   => Ok(val.clone()),
        None        => match env.parent {
            Some(ref frame) => lookup(var, *frame),
            None            => Err("Unbound variable: ".to_strbuf().append(var))
        }
    }
}

pub fn interp(sexpr: StrBuf, mut global_env: &mut Environment) -> StrBuf {

    let mut tokens = TokenStream{
        string_slice: sexpr,
        string_index: 0
    };

    let sexpr = match parse(&mut tokens, global_env) {
        Ok(good)    => good,
        Err(msg)    => return msg
    };

    let result = match eval(&sexpr, global_env) {
        Ok(good)    => good,
        Err(msg)    => return msg
    };

    result.to_str().to_strbuf()

}
