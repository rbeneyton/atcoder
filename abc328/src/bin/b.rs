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

fn is_confusing(month : usize, day : usize) -> bool {
    if month >= 10 && month / 10 != month % 10 {
        return false;
    }
    if day >= 10 && day / 10 != day % 10 {
        return false;
    }
    if month >= 10 && day >= 10 && month / 10 != day / 10 {
        return false;
    }
    month % 10 == day % 10
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_confusing() {
        assert_eq!(true ,is_confusing(1, 1));
        assert_eq!(false ,is_confusing(10, 1));
        assert_eq!(false ,is_confusing(1, 10));
        assert_eq!(false ,is_confusing(1, 2));
        assert_eq!(false ,is_confusing(1, 22));
        assert_eq!(true ,is_confusing(1, 11));
        assert_eq!(true ,is_confusing(11, 11));
        assert_eq!(true ,is_confusing(11, 1));
        assert_eq!(true ,is_confusing(11, 11));
        assert_eq!(false ,is_confusing(11, 12));
        assert_eq!(false ,is_confusing(12, 12));
        assert_eq!(false ,is_confusing(12, 1));
        assert_eq!(false ,is_confusing(12, 2));
    }
}

fn main() {
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut res = 0;
    for month in 1..=n {
        for day in 1..=d[month - 1] {
            if is_confusing(month, day) {
                res += 1;
            }
        }
    }
    println!("{res}");
}
