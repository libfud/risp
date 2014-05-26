//Translate takes a set of tokens and constructs from them an S Expression.

use super::tokenize;
use super::tokenize::{Token, TokenStream, TokenIterator};
use super::super::{SExpr, Data, Cons, Nil, DataType, Operator, Literal, Variable};

static TRANSFAIL: &'static str = "Failed to translate: malformed expression!";

pub fn translate(token: &Token) -> Result<SExpr, StrBuf> {
    match token {
        &tokenize::Operator(ref op)    => Ok(Data(Operator(op.clone()))),
        &tokenize::Literal(ref basic)  => Ok(Data(Literal(basic.clone()))),
        &tokenize::Symbol(ref symbol)  => Ok(Data(Variable(symbol.clone()))),
        &tokenize::RParen  => Ok(Nil),
        &tokenize::LParen  => Err(TRANSFAIL.to_strbuf()),
        &tokenize::Invalid => Err(TRANSFAIL.to_strbuf()),
        &tokenize::Whitespace => Err("How'd that get through?".to_strbuf())
    }
}

///Parse forms an SExpr recursively from an array of tokens
pub fn parse(mut tokens: &mut TokenStream) -> Result<(Box<SExpr>), StrBuf> {
    let car_maybe = match tokens.next() {
        Some(token) => token,
        None        => {
            return Ok(box Nil)
        }
    };

    let car = match car_maybe {
        tokenize::LParen    => try!(parse(tokens)),
        tokenize::RParen    => {
            return Ok(box Nil)
        }
        _   => box try!(translate(&car_maybe))
    };

    let cdr = try!(parse(tokens));

    match cdr {
        box Nil => Ok(car),
        _   => Ok(box Cons(car, cdr))
    }
}
