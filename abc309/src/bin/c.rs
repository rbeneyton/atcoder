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

fn solve() {
}

#[test]
fn test_solve()
{
}

// #[proconio::fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut med: [(usize, usize); n], // (days, pills)
    }
    // sort meds by increasing duration
    med.sort_by(|a, b| a.0.cmp(&b.0));
    let mut pills_per_day : usize = med.iter()
        .map(|(_, x)| x)
        .sum();
    let mut day = 1;
    for (days, pills) in med {
        if pills_per_day <= k {
            break;
        }
        logln!(1, "med:{}days/{}pills, day:{}, pills_per_day:{}", days, pills, day, pills_per_day);
        debug_assert!(day <= days);
        let rem_day = days + 1 - day;
        logln!(1, "rem_day:{}", rem_day);
        day += rem_day;
        debug_assert!(pills_per_day >= pills);
        pills_per_day -= pills;
    }
    println!("{}", day);
}
