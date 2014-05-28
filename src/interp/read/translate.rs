//Translate takes a set of tokens and constructs from them an S Expression.

use super::tokenize;
use super::tokenize::{Token, TokenStream, TokenIterator, LParen, RParen};
use super::super::{SExpr, Data, Cons, Nil, DataType, Operator, Literal, Variable};
use super::super::basictype;
use super::super::basictype::Procedure;
use super::super::operator::Lambda;
use super::super::eval::eval;
use super::super::Environment;

static TRANSFAIL: &'static str = "Failed to translate: malformed expression!";

pub fn build_lambda(mut tokens: &mut TokenStream) -> Result<SExpr, StrBuf> {
    match tokens.next() {
        Some(LParen)    => { } //good
        _   =>  return Err("Improper lambda expression!".to_strbuf())
    }

    let symbol = match tokens.next() {
        Some(tokenize::Symbol(x)) => Variable(x),
        _   => return Err("Improper lambda expression!".to_strbuf())
    };

    match tokens.next() {
        Some(RParen)    => { } //good
        _   => return Err("Improper lambda expression!".to_strbuf())
    }
    match tokens.next() {
        Some(LParen)    => { } //good
        _   => return Err("Improper lambda expression!".to_strbuf())
    }

    let car = match tokens.next() {
        Some(tokenize::Operator(op))    => super::super::Operator(op),
        _   => return Err("sorry ._.".to_strbuf())
    };


    let mut procedure = Cons(box Data(symbol), box Data(car));
    loop {
        match tokens.next() {
            Some(RParen)    => return Ok(procedure),
            Some(tokenize::Operator(op))=> procedure = Cons(box procedure, box Data(Operator(op))),
            Some(tokenize::Literal(ty)) => procedure = Cons(box procedure, box Data(Literal(ty))),
            Some(tokenize::Symbol(sym)) => procedure = Cons(box procedure, box Data(Variable(sym))),
            _   => return Err("Nope".to_strbuf())
        }
    }
}

///Parse forms an SExpr recursively from an array of tokens
pub fn parse(mut tokens: &mut TokenStream, mut environment: &mut Environment) 
                                            -> Result<Vec<DataType>, StrBuf> {
    let mut terms: Vec<DataType> = Vec::new();

    loop {
        let token = tokens.next();
        match token {
            Some(RParen)    => return Ok(terms),
            Some(LParen)    => {
                let inner_terms = try!(parse(tokens, environment));
                let term = try!(eval(&inner_terms, environment));
                terms.push(Literal(term));
            },
            Some(tokenize::Operator(op))=> match op {
                Lambda  => {
                    terms.push(Operator(Lambda));
                    let lambda = try!(build_lambda(tokens));
                    terms.push(Literal(Procedure(box lambda)));
                },
                _       => terms.push(Operator(op)),
            },
            Some(tokenize::Literal(ty)) => terms.push(Literal(ty)),
            Some(tokenize::Symbol(x))   => terms.push(Variable(x)),
            None    => return Ok(terms),
            _   => {
                println!("{}", token);
                return Err(TRANSFAIL.to_strbuf())
            }
        }
    }
}
