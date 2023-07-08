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

fn solve(n: usize, a: Vec::<bool>) -> Vec::<bool> {
    let idx = |row, col| {
        debug_assert!(row >= 1 && row <= n);
        debug_assert!(col >= 1 && col <= n);
        (row - 1) * n + (col - 1)
    };

    let mut res = a.clone(); // no in place moves
    for i in 1..=n {
        // rows shift
        if i != 1 { res[idx(1, i)] = a[idx(1, i - 1)]; }
        if i != n { res[idx(n, i)] = a[idx(n, i + 1)]; }
        // cols shift
        if i != n { res[idx(i, 1)] = a[idx(i + 1, 1)]; }
        if i != 1 { res[idx(i, n)] = a[idx(i - 1, n)]; }
    }

    res
}

#[test]
fn test_solve()
{
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [Bytes; n],
    }
    let a = a.iter()
        .flat_map(|x| x.iter())
        .map(|x| *x == b'1')
        .collect::<Vec<bool>>();
    let res = solve(n, a);
    let idx = |row, col| row * n + col;
    for row in 0..n {
        for col in 0..n {
            print!("{}", if res[idx(row, col)] { "1" } else { "0" });
        }
        println!("");
    }
}
