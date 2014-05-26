#![crate_id = "risp"]
#![crate_type = "bin"]

//! A Lisp interpreter.

extern crate libc;
extern crate getopts;
extern crate collections;

use std::os;
use libc::c_char;
use std::c_str::CString;
use interp::{interp, Environment};
use getopts::{optopt, getopts};
use collections::HashMap;

pub mod interp;

#[link(name = "linenoise")]
extern {
    fn linenoise(p: *c_char) -> *c_char;
    fn linenoiseHistoryAdd(l: *c_char);
}

/// Attempts to read input from a user using linenoise. Returns an option,
/// Some(StrBuf) for success, or None if EOF (^D) is entered.
pub fn rust_linenoise(prompt: &str) -> Option<StrBuf> {
    if prompt.len() == 0 {
        return None
    }

    let c_prompt = prompt.to_c_str();

    c_prompt.with_ref(|c_buf| {
        unsafe {
            let ret_str = CString::new(linenoise(c_buf), true);
            if ret_str.is_not_null() {
                ret_str.as_str().map(|ret_str| ret_str.to_strbuf())
            } else {
                None
            }
        }
    })
}

/// Adds a string to a history buffer.
pub fn rust_add_history(line: &str) {
    if line.len() == 0 {
        return
    }

    let c_line = line.to_c_str();
    c_line.with_ref(|c_line| {
        unsafe {
            linenoiseHistoryAdd(c_line);
        }
    });
}

fn main() {
    let mut global_env = Environment{ variables: HashMap::new() };
    let args: Vec<StrBuf> = os::args().iter().map(|x| x.to_strbuf()).collect();

    let program = args.get(0).clone();

    let opts = [
        optopt("n", "noninteractive", "non-interactive mode", "INPUT STRING"),
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m)   => { m }
        Err(f)  => { fail!(f.to_err_msg()) }
    };

    if matches.opt_present("n") {
        match matches.opt_str("n") {
            Some(sexpr) => {
                let msg = interp(sexpr.to_str(), &global_env);
                println!("{}", msg);
                return
            }
            None    => {
                println!("Bad expr");
                return
            }
        }
    }
                

    loop {
        let mut expr = match rust_linenoise(">>> ") {
            Some(val)   => { val.as_slice().trim().to_str() },
            None    => { "".to_str() } //I hate ~str 
        };

        let mut okay_expr = true;

        if expr.as_slice().starts_with("(") || expr.len() == 0 {
            let count_parens = |expr: &str| -> (uint, uint, bool) {
                let (mut lparens, mut rparens) = (0, 0);
                for c in expr.chars() {
                    if c == '(' {
                        if lparens == rparens && lparens != 0 {
                            return (lparens, rparens, false)
                        } else {
                            lparens += 1
                        }
                    } else if c == ')' {
                        if rparens >= lparens && rparens != 0 {
                            return (lparens, rparens, false)
                        } else {
                            rparens += 1
                        }
                    }
                }

                (lparens, rparens, true)
            };

            loop {
                let (lparens, rparens, okay_expr_tmp) = count_parens(expr.as_slice());
                if lparens == 0 && rparens == 0 && expr.len() > 1 {
                    break
                }

                if okay_expr_tmp == false {
                    okay_expr = false;
                    break
                }
                if lparens == rparens && lparens != 0 {
                    break
                }

                let expr_part = match rust_linenoise(" ") {
                    Some(val)   => { val.to_str() },
                    None    => { continue }
                };
                expr = expr.append("\n");
                expr = expr.append(expr_part.as_slice());
            }
        }

        if okay_expr == false {
            println!("Bad expression");
            continue
        }
        rust_add_history(expr.as_slice());

        match expr.as_slice() {
            "(exit)" | "exit" | ",q"    => { break },
            _   => { }
        }

        let msg = interp(expr.as_slice().trim().to_str(), &global_env);
        //holy hell I hate this
        println!("{}", msg);
    }
}
