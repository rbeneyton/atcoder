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

fn solve_a(mut n : u64, k : u64) -> bool {
    logln!(1, "n:{} k:{}", n, k);
    let mut ki = 0;
    let mut idx = 0;
    loop {
        let rem = n % 3;
        ki += rem; // [1|2] * 3**0
        logln!(1, "n:{} idx:{} rem:{} k:{}/{}", n, idx, rem, ki, k);
        n /= 3;
        idx += 1;
        if n == 0 {
            if ki > k { break false; }
            if ki == k { break true; }
            // XXX we can exchange 1*3**n by 3*3**(n-1)
            // so require that difference between ki and k is multiple of 2
            break (k - ki) % 2 == 0;
        }
    }
}

#[test]
fn test_solve_a()
{
    assert_eq!(solve_a(5, 3), true);

    assert_eq!(solve_a(5, 2), false);
    assert_eq!(solve_a(5, 4), false);

    assert_eq!(solve_a(17, 2), false);
    assert_eq!(solve_a(163, 79), true);
    assert_eq!(solve_a(1000000000000000000, 1000000000000000000), true);
}

// #[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: u64,
            k: u64,
        }
        println!("{}", if solve_a(n, k) { "Yes" } else { "No" });
    }
}
