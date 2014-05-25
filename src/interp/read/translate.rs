//Translate takes a set of tokens and constructs from them an S Expression.

use super::tokenize;
use super::tokenize::Token;
use super::super::{SExpr, Data, Cons, Nil, DataType, Operator, Literal, Variable};

static TRANSFAIL: &'static str = "Failed to translate: malformed expression!";

pub fn translate(token: &Token) -> Result<SExpr, StrBuf> {
    match token {
        &tokenize::Operator(ref op)    => Ok(Data(Operator(op.clone()))),
        &tokenize::Literal(ref basic)  => Ok(Data(Literal(basic.clone()))),
        &tokenize::Symbol(ref symbol)  => Ok(Data(Variable(symbol.clone()))),
        &tokenize::RParen  => Ok(Nil),
        &tokenize::LParen  => Err(TRANSFAIL.to_strbuf()),
    }
}

///Finds the length of a subexpression, with the lparen cut off. For example, given
/// + 2 (* 3 4) 7) 9), it should return 10 (the rparen following the 7)
pub fn find_rparen(tokens: &[Token]) -> Result<uint, StrBuf> {
    let mut lparens = 1u;
    let mut rparens = 0u;
    let mut counter = 0;

    for token in tokens.iter(){
        match token {
            &tokenize::RParen => {
                rparens += 1;
                if rparens == lparens {
                    break
                }
            },
            &tokenize::LParen => lparens += 1,
            _       => { } //do nothing
        }
        counter += 1;
    }

    if rparens == lparens {
        Ok(counter)
    } else { //fewer rparens than lparens
        Err("Couldn't find closing RParens!".to_strbuf())
    }
}
 
///Parse forms an SExpr recursively from an array of tokens
pub fn parse(tokens: Iterator) -> Result<Box<SExpr>, StrBuf> {
    if tokens.len() == 0 { //shouldn't be fed a 0 length array
        return Err(TRANSFAIL.to_strbuf())    
    }

    if tokens.len() == 1 { //atom
        let mut car = match translate(&tokens[0]) {
            Ok(good)    => good,
            Err(msg)    => return Err(msg.to_strbuf())
        };
    
        Ok(box Cons(box car, box Nil))
    } else if tokens.len() == 2 {
        let mut car = match translate(&tokens[0]) {
            Ok(good)    => good,
            Err(msg)    => return Err(msg.to_strbuf())
        };

        let cdr = match translate(&tokens[1]) {
            Ok(good)    => good,
            Err(msg)    => {
                return Err(msg.to_strbuf())
            }
        };
        Ok(box Cons(box car, box cdr))
    } else {
        //Car is an SExpr, so we should properly construct it
        match tokens[0] == tokenize::LParen {
            true    => {
                let sub_expr_len = try!(find_rparen(tokens.slice_from(1)));
                let car = try!(parse(tokens.slice(1, sub_expr_len + 1)));
                let cdr = try!(parse(tokens.slice_from(sub_expr_len + 2)));

                Ok(box Cons(car, cdr))
            }
            false   => {
                let car = box try!(translate(&tokens[0]));
                let cdr = try!(parse(tokens.slice_from(1)));

                Ok(box Cons(car, cdr))
            }
        }
    }
}
