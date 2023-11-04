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

fn main() {
    input! {
        a: [[usize; 9]; 9],
    }
    for i in 0..9 {
        let row = &a[i];
        let mut seen = [false; 10];
        for j in 0..9 {
            let v = row[j];
            debug_assert!(v < 10);
            if seen[v] {
                println!("No");
                return;
            }
            seen[v] = true;
        }
    }
    for i in 0..9 {
        let mut seen = [false; 10];
        for j in 0..9 {
            let v = a[j][i];
            debug_assert!(v < 10);
            if seen[v] {
                println!("No");
                return;
            }
            seen[v] = true;
        }
    }
    for i in (0..3).map(|x| x * 3) {
        for j in (0..3).map(|x| x * 3) {
            let mut seen = [false; 10];
            for di in 0..3 {
                for dj in 0..3 {
                    let v = a[i + dj][j + di];
                    debug_assert!(v < 10);
                    if seen[v] {
                        println!("No");
                        return;
                    }
                    seen[v] = true;
                }
            }
        }
    }
    println!("Yes");
}
