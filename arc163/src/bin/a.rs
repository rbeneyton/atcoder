// {{{ usual stuff

#![allow(unused_imports)]
#![allow(unused_macros)]

use std::{
    collections::*,
    fmt,
    io::{self, Write},
    mem::swap,
};
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Bytes, Chars};

#[allow(dead_code)]
#[allow(unused_variables)]
const LOG_LVL : u8 = 0;

macro_rules! logstart {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("L{}:L{}:{}",
                line!(),
                lvl,
                fmt::format(format_args!($($arg)+)));
        }
    })
}
macro_rules! logcont {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
macro_rules! logstop {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprintln!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
macro_rules! logln {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprintln!("L{}:L{}:{}",
                line!(),
                lvl,
                fmt::format(format_args!($($arg)+)));
            io::stderr().flush().unwrap();
        }
    })
}

pub fn get_stdin_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

// }}}

fn is_lower(a : &[char], b : &[char]) -> bool {
    a < b // slice Ord: Implements comparison of vectors lexicographically.
}

#[test]
fn test_is_lower()
{
    assert_eq!(is_lower(&['a'], &['a', 'a']), true);
    assert_eq!(is_lower(&['a'], &['a', 'b']), true);
    assert_eq!(is_lower(&['a', 'b'], &['a', 'b']), false);
    assert_eq!(is_lower(&['a', 'b'], &['a', 'b', 'z', 'z', 'z']), true);
    assert_eq!(is_lower(&['a', 'b'], &['a', 'c']), true);
    assert_eq!(is_lower(&['a', 'b'], &['a', 'c', 'z']), true);
}

fn is_divide(v : &[char]) -> bool {
    let n = v.len();
    if n == 1 {
        return true;
    }
    for sz in 1..n {
        if is_lower(&v[0..sz], &v[sz..]) {
            return true;
        }
    }

    false
}

#[test]
fn test_is_divide()
{
    let is_divide_str = |s : &str| {
        let v : Vec<_> = s.chars().collect();
        is_divide(&v)
    };
    assert_eq!(is_divide_str("abac"), true);
    assert_eq!(is_divide_str("cac"), false);
    assert_eq!(is_divide_str("ab"), true);
    assert_eq!(is_divide_str("abababababab"), true);
    assert_eq!(is_divide_str("edcba"), false);
}

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
        }
        debug_assert_eq!(n, s.len());
        for c in &s {
            debug_assert!(c.is_ascii_lowercase());
        }
        let res = is_divide(&s);
        println!("{}", if res { "Yes" } else { "No" });
    }
}
