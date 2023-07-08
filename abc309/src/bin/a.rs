// {{{ usual stuff

#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

use std::{
    collections::*,
    fmt,
    io::{self, Write},
    mem::swap,
};
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Bytes, Chars};

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

fn solve(a: i8, b: i8) -> bool {
    let row_a = (a - 1) / 3;
    let row_b = (b - 1) / 3;
    let adjacent = row_a == row_b;

    let col_a = (a - 1) % 3;
    let col_b = (b - 1) % 3;
    let d_col = (col_a - col_b).abs();

    adjacent && (d_col == 1)
}

#[test]
fn test_solve()
{
    assert_eq!(solve(1, 2), true);
    assert_eq!(solve(1, 3), false);
    assert_eq!(solve(1, 4), false);
    assert_eq!(solve(1, 7), false);

    assert_eq!(solve(7, 8), true);
    assert_eq!(solve(1, 9), false);
    assert_eq!(solve(3, 4), false);
}

#[proconio::fastout]
fn main() {
    input! {
        a: i8,
        b: i8,
    }
    let res = solve(a, b);
    println!("{}", if res { "Yes" } else { "No" });
}
