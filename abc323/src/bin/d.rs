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
        n: usize,
        sc: [(usize, usize); n], // (size, count)
    }
    let mut sc = sc.iter().cloned().collect::<BTreeMap::<_, _>>();
    let mut thr = *sc.first_key_value().unwrap().0;
    loop {
        // take smaller size with count >= 2
        let small = sc.range(thr..).filter(|(_, c)| **c >= 2).next();
        if small.is_none() { break; }
        let (small, small_c) = small.unwrap();
        let (small, small_c) = (*small, *small_c);
        let (moved, rem) = num_integer::div_rem(small_c, 2);
        sc.entry(small).and_modify(|x| *x = rem);
        sc.entry(2 * small).and_modify(|x| *x += moved).or_insert(moved);
        thr = small;
    }
    println!("{}", sc.values().sum::<usize>());
}
