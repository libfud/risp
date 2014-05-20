//! Tokenizes input strings.

use super::{SExpr, Cons, Nil, DataType, Operator, Literal, car, cdr};
use super::basictype::{BasicType, Boolean, Character, Number, String, Symbol};
use super::operator::OperatorType;

pub enum Token {
    LParen,
    RParen,
    Operator(OperatorType),
    Literal(BasicType)
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>>, StrBuf> {
    let mut tokens = Vec::new();

    let mut i = 0;
    let len = expr.len();

    while i < len {
        let slice = expr.slice_from(i);

        if slice.chars().next().unwrap().is_whitespace() {
            i += 1;
            continue;
        }

        //Parentheses
        let token = match slice.chars().next().unwrap() {
            '(' => Some(LParen),
            ')' => Some(RParen),
            _   => None
        };
        if token.is_some() {
            tokens.push(token.unwrap());
            i += 1;
            continue;
        }

        //operators
        //there's at least one word, so it's safe to unwrap
        let word = slice.words().next().unwrap();
        
        //Discard dangling parens
        let word = word.slice(0, word.find(|c: char| c == ')').unwrap_or(word.len()));

        match operator::From_str(word) {
            Some(op_type)   => {
                tokens.push(Operator(op_type));
                i += word.len();
                continue;
            }
            _               => { } //do nothing
        };

        //Booleans
        match word {
            "true"  => {
                tokens.push(Literal(Boolean(true)));
                i += word.len();
                continue;
            },
            "false" => {
                tokens.push(Literal(Boolean(false)));
                i += word.len();
                continue;
            },
            _       => { } // do nothing
        }

        //Character literals
        if word.len() == 3 && word.starts_with("'") && word.ends_with("'") {
            tokens.push(Literal(Character(word.chars().next().next().unwrap())));
        }
