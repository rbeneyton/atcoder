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

fn solve(n: usize, m: usize) -> usize {
    // todo!();

    0
}

#[test]
fn test_solve()
{
    assert_eq!(true, true);
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let res = solve(n, m);
    logln!(0, "toto");
    println!("{}", res);
}
