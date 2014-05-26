//! Tokenizes input strings.

use super::super::basictype::{BasicType, Boolean, Character, Number, String};
use super::super::basictype::{Floating, Integer, UInteger};
use super::super::operator::OperatorType;
use super::super::operator;

#[deriving(Show)]
#[deriving(Eq)]
pub enum Token {
    LParen,
    RParen,
    Operator(OperatorType),
    Literal(BasicType),
    Symbol(StrBuf), //Can be a variable or part of an enumeration
    Invalid,
    Whitespace //only used internally in iterators
}

#[deriving(Copy, Clone)]
pub struct TokenStream {
    pub string_slice: StrBuf,
    pub string_index: uint,
}

pub trait TokenIterator<TokenStream> {
    fn next_token_type(&self) -> (Option<Token>, uint);
    fn next(&mut self) -> Option<Token>;
    fn peek(&mut self) -> Option<Token>; //pesky whitespace
}

impl TokenIterator<Token> for TokenStream {
    fn next_token_type(&self) -> (Option<Token>, uint) {
        if self.string_slice.len() - self.string_index == 0 {
            (None, 0)
        } else {
            match self.string_slice.as_slice().slice_from(
                                    self.string_index).chars().next().unwrap() {
                '(' => {
                    return (Some(LParen), 1)
                },
                ')' => {
                    return (Some(RParen), 1)
                },
                ' ' => {
                    return (Some(Whitespace), 1)
                },
                _   => {},
            }

            let word = self.string_slice.as_slice().slice_from(
                                    self.string_index).words().next().unwrap();
            //discard dangling parens
            let word = word.slice(0, word.find(|c: char| c == ')' || c == '(').unwrap_or(word.len()));

            match operator::from_str(word) {
                Some(op_type) => {
                    return (Some(Operator(op_type)), word.len())
                }
                None    => { } //do nothing
            }

            match word {
                "#t"    => {
                    return (Some(Literal(Boolean(true))), word.len())
                }
                "#f"    => {
                    return (Some(Literal(Boolean(false))), word.len())
                }
                _       => { } //do nothing
            }

            //Character literals
            if word.len() >= 2 && word.starts_with("#\\") {
                match word.len() {
                    2   => {
                        return (Some(Literal(Character(' '))), word.len())
                    }
                    3   => {
                        return (Some(Literal(Character(word.chars().nth(2).unwrap()))), word.len())
                    }
                    _   => match word.slice_from(2) {
                        "space"     => {
                            return (Some(Literal(Character(' '))), word.len())
                        }
                        "newline"   => {
                            return (Some(Literal(Character('\n'))), word.len())
                        }
                        _           => { }
                    }
                }
            }

            //strings
            if word.starts_with("\"") {
                let str_len = match self.string_slice.as_slice().slice_from(self.string_index
                                                                + 1).find(|c: char| c == '\"') {
                    Some(x) => x,
                    None    => {
                        return (Some(Invalid), 0)
                    }
                };
                //add one to account for the quote and one more to get to the next
                //character not in the string
                return (Some(Literal(String(self.string_slice.as_slice().slice(self.string_index + 1, 
                                str_len).to_strbuf()))), str_len + 2)
            }

            //symbols and identifiers
            match word.chars().next().unwrap() {
                'a'..'z'|'A'..'Z'|'_'|'*'   => {
                    return (Some(Symbol(word.to_strbuf())), word.len())
                }
                _   => { }
            }

            //Numeric literals
            let mut neg_counter = 0;
            let mut radix_counter = 0;
            let mut literal_flag = false;
            let mut literal_spec = 'i';

            for c in word.chars() {
                match c {
                    '0'..'9'    => { }
                    '-'         => neg_counter += 1,
                    '.'         => radix_counter += 1,
                    'i'|'u'|'f' => {
                        if literal_flag == true {
                            return (Some(Invalid), 0)
                        } else {
                            literal_flag = true;
                            literal_spec = c;
                        }
                    }
                    _           => {
                        return (Some(Invalid), 0)
                    }
                }
            }

            //formatted literal
            if literal_flag == true {
                let token = match literal_spec {
                    'i' => match from_str::<int>(word) {
                        Some(x) => Some(Literal(Number(Integer(x)))),
                        None    => Some(Invalid)
                    },
                    'u' => match from_str::<uint>(word) {
                        Some(x) => Some(Literal(Number(UInteger(x)))),
                        None    => Some(Invalid)
                    },
                    'f' => match from_str::<f64>(word) {
                        Some(x) => Some(Literal(Number(Floating(x)))),
                        None    => Some(Invalid)
                    },
                    _   => Some(Invalid)
                };

                return (token, word.len())
            }

            //time to play guess the type
            let token = match (neg_counter, radix_counter) {
                (1, 0)  => {
                    match from_str::<int>(word) {
                        Some(x) => Some(Literal(Number(Integer(x)))),
                        None    => Some(Invalid)
                    }
                }
                (0, 0)  => {
                    match from_str::<uint>(word) {
                        Some(x) => Some(Literal(Number(UInteger(x)))),
                        None    => Some(Invalid)
                    }
                }
                (0, 1) | (1, 1) => {
                    match from_str::<f64>(word) {
                        Some(x) => Some(Literal(Number(Floating(x)))),
                        None    => Some(Invalid)
                    }
                }

                _   => Some(Invalid)
            };

            return (token, word.len())
        }
    }

    fn next(&mut self) -> Option<Token> {
        loop {
            let (token, advance) = self.next_token_type();
            self.string_index += advance;
            if token != Some(Whitespace) { //guaranteed on None.
                return token
            }
        }
    }

    fn peek(&mut self) -> Option<Token> {
        loop {
            let (token, advance) = self.next_token_type();
            if token != Some(Whitespace) {
                return token
            } else {
                self.string_index += advance
            }
        }
    }
}
