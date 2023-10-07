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
        m: usize,
        a: [usize; m],
        s: [Chars; n], // Chars of size m
    }
    for i in 0..n {
        debug_assert_eq!(s[i].len(), m);
    }
    let scores = (0..n)
        .map(|i| s[i].iter()
            .enumerate()
            .map(|(idx, c)| if *c == 'o' { a[idx] } else { 0 })
            .sum::<usize>()
            + i // bonus
        )
        .collect::<Vec<_>>();
    // dbg!(&scores);
    let max_score = *scores.iter().max().unwrap();
    // dbg!(&max_score);
    'player: for i in 0..n {
        // sort remaining problems by decreasing points
        let mut rems = s[i].iter()
            .enumerate()
            .filter_map(|(idx, c)| if *c == 'x' { Some(a[idx]) } else { None })
            .collect::<Vec<_>>();
        rems.sort_by(|a, b| b.cmp(&a)); // reverse
        // reach target max_score
        let mut score = scores[i];
        for (j, remi) in rems.iter().enumerate() {
            if score >= max_score {
                println!("{}", j);
                continue 'player;
            }
            score += remi;
        }
        println!("{}", rems.len());
    }
}
