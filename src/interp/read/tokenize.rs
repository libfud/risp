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
}

pub struct TokenStream {
    slice: &str,
}

impl Iterator<Token> for TokenStream {
    fn next(&mut self) -> Option<Token> {
        if self.slice == 0 {
            None
        } else {
            loop {
                if self.slice.chars().next().unwrap().is_whitespace() {
                    self.slice = self.slice.slice_from(1);
                    continue;
                } else {
                    let token = match self.slice.chars().next().unwrap() {
                        '(' => Some(LParen),
                        ')' => Some(RParen),
                        _   => None,
                    };
                    if token.is_some() {
                        self.slice = self.slice.slice_from(1);
                        return Some(token)
                    }

                    let word = self.slice.words().next().unwrap();
                    //discard dangling parens
                    let word = word.slice(0, word.find(|c: char| c == ')').unwrap_or(word.len()));

                    match operator::from_str(word) {
                        Some(op_type) => {
                            self.slice = self.slice.slice_from(word.len());
                            return Some(Operator(op_type))
                        }
                        None    => { } //do nothing
                    }

                    match word {
                        "#t"    => {
                            self.slice = self.slice.slice_from(word.len());
                            return Some(Literal(Boolean(true)))
                        }
                        "#f"    => {
                            self.slice = self.slice.slice_from(word.len());
                            return Some(Literal(Boolean(false)))
                        }
                        _       => { } //do nothing
                    }

                    //Character literals
                    if word.len() >= 2 && word.starts_with("#\\") {
                        match word.len() {
                            2   => {
                                self.slice = self.slice.slice_from(word.len());
                                return Some(Literal(Character(' ')))
                            }
                            3   => {
                                self.slice = self.slice.slice_from(word.len());
                                return Some(Literal(Character(word.chars().nth(2).unwrap())))
                            }
                            _   => match word.slice_from(2) {
                                "space"     => {
                                    self.slice = self.slice.slice_from(word.len());
                                    return Some(Literal(Character(' ')))
                                }
                                "newline"   => {
                                    self.slice = self.slice.slice_from(word.len());
                                    return Some(Literal(Character('\n')))
                                }
                                _           => { }
                            }
                        }
                    }

                    //strings
                    if word.starts_with("\"") {
                        let str_len = match self.slice.slice_from(1).find(|c: char| c == '\"') {
                            Some(x) => x,
                            None    => {
                                return Some(Invalid)
                            }
                        };
                        self.slice = self.slice.slice_from(str_len + 2);
                        //add one to account for the quote and one more to get to the next
                        //character not in the string
                        return Some(Literal(String(self.slice.slice(1, str_len))))
                        //avoid the first quote and the last quote
                    }

                    //symbols and identifiers
                    match word.chars().next().unwrap() {
                        'a'..'z'|'A'..'Z'|'_'|'*'   => {
                            self.slice = self.slice.slice_from(word.len());
                            return Some(Symbol(word.to_strbuf()))
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
                                    return Some(Invalid)
                                } else {
                                    literal_flag = true;
                                    literal_spec = c;
                                }
                            }
                            _           => {
                                return Some(Invalid)
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
                            'u' => match from_str::<int>(word) {
                                Some(x) => Some(Literal(Number(UInteger(x)))),
                                None    => Some(Invalid)
                            },
                            'f' => match from_str::<f64>(word) {
                                Some(x) => Some(Literal(Number(Floating(x)))),
                                None    => Some(Invalid)
                            },
                            _   => Some(Invalid)
                        };

                        self.slice = self.slice.slice_from(word.len());
                        return token
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

                    self.slice = self.slice.slice_from(word.len());
                    return token
                }
            }
        }
    }
}
