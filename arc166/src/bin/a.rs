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

// #[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    'testcase: for _ in 0..t {
        input! {
            n: usize,
            src: Chars, // x
            dst: Chars, // y
        }
        debug_assert_eq!(src.len(), n);
        debug_assert_eq!(dst.len(), n);

        // C cannot be generated
        for i in 0..n {
            if dst[i] == 'C' && src[i] != 'C' {
                println!("No");
                continue 'testcase;
            }
        }

        // A's move only right
        let (mut as_in_src_min, mut as_in_src_max, mut as_in_dst) = (0, 0, 0);
        for i in 0..n {
            if src[i] == 'A' {
                as_in_src_min += 1;
            }
            if src[i] == 'A' || (src[i] == 'C' && dst[i] != 'C') {
                as_in_src_max += 1;
            }
            if dst[i] == 'A' {
                as_in_dst += 1;
            }
            if as_in_dst > as_in_src_max {
                println!("No");
                continue 'testcase;
            }
            if dst[i] == 'C' || i == n - 1 {
                if as_in_dst < as_in_src_min {
                    println!("No");
                    continue 'testcase;
                }
            }
            // reset
            if dst[i] == 'C' {
                as_in_src_min = 0;
                as_in_src_max = 0;
                as_in_dst = 0;
            }
        }

        println!("Yes");
    }
}
