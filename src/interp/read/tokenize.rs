//! Tokenizes input strings.

use super::super::basictype::{BasicType, Boolean, Character, Number, String};
use super::super::basictype::{Floating, Integer};
use super::super::operator::OperatorType;
use super::super::operator;

#[deriving(Show)]
pub enum Token {
    LParen,
    RParen,
    Operator(OperatorType),
    Literal(BasicType),
    Symbol(StrBuf)
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, StrBuf> {
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

        match operator::from_str(word) {
            Some(op_type)   => {
                tokens.push(Operator(op_type));
                i += word.len();
                continue;
            }
            _               => { } //do nothing
        };

        //Booleans
        match word {
            "#t"  => {
                tokens.push(Literal(Boolean(true)));
                i += word.len();
                continue;
            },
            "#f" => {
                tokens.push(Literal(Boolean(false)));
                i += word.len();
                continue;
            },
            _       => { } // do nothing
        }

        //Character literals
        if word.len() >= 2 && word.starts_with("#\\") {
            match word.len() {
                2   => tokens.push(Literal(Character(' '))),
                3   => tokens.push(Literal(Character(word.chars().nth(2).unwrap()))),
                _   => match word.slice_from(2) {
                    "space"     => tokens.push(Literal(Character(' '))),
                    "newline"   => tokens.push(Literal(Character('\n'))),
                    _           => {}
                }
            }

            i += word.len();
            continue;
        }

        //String literals
        if word.starts_with("\"") {
            let str_len = match slice.slice_from(1).find(|c: char| c == '\"') {
                Some(x) => x,
                None    => {
                    return Err("Unterminated quote!".to_strbuf())
                }
            };
           
            tokens.push(Literal(String(slice.slice(1, str_len).to_strbuf())));
            i += str_len + 2; //add one to get to the next double quote, add one to escape that
            continue;
        }

        //Symbols/variables
        match word.chars().next().unwrap() {
            'a'..'z'|'A'..'Z'|'_'|'*'   => { 
                tokens.push(Symbol(word.to_strbuf()));
                i += word.len();
                continue;
            }
            _   => {} //do nothing
        };

        //Numeric literals (only machine words for now)
        let mut negative_counter = 0;
        let mut radix_point_counter = 0;

        for c in word.chars() {
            match c {
                '0'..'9'    => { }, //looks numeric to me
                '-'         => { negative_counter += 1 },
                '.'         => { radix_point_counter += 1 },
                _           => { return Err(("Unrecognized token".to_strbuf())) }
            }
        }

        match (negative_counter, radix_point_counter) {
            (0, 0) | (1, 0) => {
                match from_str::<int>(word) {
                    Some(x) => {
                        tokens.push(Literal(Number(Integer(x))));
                        i += word.len();
                        continue;
                    }
                    None    => {
                        return Err("Misplaced negative sign!".to_strbuf())
                    }
                }
            },

            (0, 1) | (1, 1) => {
                match from_str::<f64>(word) {
                    Some(x) => {
                        tokens.push(Literal(Number(Floating(x))));
                        i += word.len();
                        continue;
                    }
                    None    => {
                        return Err("Misplaced negative sign!".to_strbuf())
                    }
                }
            },

            _   => { return Err("Too many negative signs and/or points!".to_strbuf()) }
        }
    }

    Ok(tokens)
}
