#![allow(non_upper_case_globals)]
#![allow(unused_macros)]

use itertools::Itertools;
// use std::io::prelude::*;
// use std::fmt;
// use std::cmp;
// use std::collections::BTreeMap;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

#[allow(dead_code)]
#[allow(unused_variables)]
const LOG_LVL : u8 = 100;

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
#[macro_export]
macro_rules! logcont {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
#[macro_export]
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
            std::io::stderr().flush().unwrap();
        }
    })
}

fn main() {
    let (n, l) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let a = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(a.len(), n);

    let group_1_n = a.iter().filter(|x| **x == 1).count();
    let group_2_n = a.iter().filter(|x| **x == 2).count();
    assert_eq!(group_1_n + group_2_n, n);
    assert_eq!(group_1_n + 2 * group_2_n, l);

    let mut remain = l as isize;
    for i in a.iter() {
        let i = *i as isize;
        if i == 2 && remain < i {
            println!("No");
            return;
        }
        remain -= 1 + i; // optimal waste
    }
    // guaranteed that all N groups can take seats.
    println!("Yes");
}
