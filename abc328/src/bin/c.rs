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
        n: usize,
        q: usize,
        s: Chars,
    }
    debug_assert_eq!(s.len(), n);

    let mut doubles = Vec::with_capacity(n);
    let mut acc_doubles = 0;
    doubles.push((false, acc_doubles));
    for ((idx1, c1), (idx2, c2)) in s.iter().enumerate().tuple_windows() {
        debug_assert_eq!(idx2, idx1 + 1);
        let double = c1 == c2;
        acc_doubles += double as usize;
        doubles.push((double, acc_doubles));
    }
    debug_assert_eq!(doubles.len(), n);

    // eprintln!("{}", itertools::join(
    //             doubles.iter().enumerate().map(|(idx, x)| format!("{}:{}/{}", idx, x.0, x.1)),
    //             ","));
    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }
        let (l, r) = (l - 1, r - 1);
        debug_assert!(l < n);
        debug_assert!(r < n);
        //eprintln!("{}:{}/{}", l, doubles[l].0, doubles[l].1);
        //eprintln!("{}:{}/{}", r, doubles[r].0, doubles[r].1);
        let mut inner_doubles = doubles[r].1 - doubles[l].1;
        // if doubles[l].0 { inner_doubles -= 1; }
        println!("{inner_doubles}");
    }
}
