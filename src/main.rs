#![crate_id = "risp"]
#![crate_type = "bin"]

//! A Lisp interpreter.

extern crate libc;

use libc::c_char;
use std::c_str::Cstring;

#[link(name = "readline")]
extern {
    fn readline(p: *c_char) -> *c_char;
    fn add_history(l: *c_char);
}

/// Attempts to read input from a user using readline. Returns an option,
/// Some(StrBuf) for success, or None if EOF (^D) is entered.
pub fn rust_readline(prompt: &str) -> Option<StrBuf>
{
    if prompt.len() == 0 {
        return None
    }

    let c_prompt = prompt.to_c_str();

    c_prompt.with_ref(|c_buf| {
        unsafe {
            let ret_str = CString::new(readline(c_buf), true);
            if ret_str.is_not_null() {
                ret_str.as_str().map(|ret_str| ret_str.to_strbuf())
            } else {
                None
            }
        }
    })
}

/// Adds a string to a readline history.
pub fn rust_add_history(line: &str) {
    if line.len() == 0 {
        return
    }

    let c_line = line.to_c_str();
    c_line.with_ref(|c_line| {
        unsafe {
            add_history(c_line);
        }
    });
}

fn main()
{
    loop {
        let expr = match rust_readline(">>> ") {
            Some(val)   => { val.to_str() },
            None    => { continue }
        };
        rust_add_histor(expr);

        match expr.trim() {
            "(exit)" | "exit" | ",q"    => { break },
            _   => { println!("{}", expr); }
        }
    }
}
