//Translate takes a set of tokens and constructs from them an S Expression.

use super::tokenize;
use super::tokenize::{Token, TokenStream, TokenIterator, LParen, RParen};
use super::super::{SExpr, Data, Cons, Nil, DataType, Operator, Literal, Variable};
use super::super::basictype;
use super::super::eval::eval;
use super::super::Environment;

static TRANSFAIL: &'static str = "Failed to translate: malformed expression!";

///Parse forms an SExpr recursively from an array of tokens
pub fn parse(mut tokens: &mut TokenStream, mut environment: &Environment) 
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
            Some(tokenize::Operator(op))=> terms.push(Operator(op)),
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
