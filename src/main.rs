#![crate_id = "risp"]
#![crate_type = "bin"]

//! A Lisp interpreter.

extern crate libc;

use libc::c_char;
use std::c_str::CString;

#[link(name = "linenoise")]
extern {
    fn linenoise(p: *c_char) -> *c_char;
    fn linenoiseHistoryAdd(l: *c_char);
}

/// Attempts to read input from a user using linenoise. Returns an option,
/// Some(StrBuf) for success, or None if EOF (^D) is entered.
pub fn rust_linenoise(prompt: &str) -> Option<StrBuf>
{
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

fn main()
{
    loop {
        let expr = match rust_linenoise(">>> ") {
            Some(val)   => { val.to_str() },
            None    => { continue }
        };
        rust_add_history(expr);

        match expr.trim() {
            "(exit)" | "exit" | ",q"    => { break },
            _   => { println!("{}", expr); }
        }
    }
}
