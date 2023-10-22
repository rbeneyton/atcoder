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
        h: isize, // h rows
        w: isize, // w columns
    }
    let n = h * w;
    let mut map = Vec::new();
    map.resize(n as usize, 0usize);
    let idx = |row, col| {
        debug_assert!(row >= 0 && row < h);
        debug_assert!(col >= 0 && col < w);
        (row * w + col) as usize
    };
    let mut nc = 1;
    for row in 0..h {
        input! {
            line: Chars,
        }
        debug_assert_eq!(line.len(), w as usize);
        for col in 0..w {
            if line[col as usize] == '#' {
                map[idx(row, col)] = nc;
                nc += 1;
            }
        }
    }
    dbg!(&nc);

    let mut clusters = (0..nc).collect::<Vec<_>>();
    loop {
        let mut moved = 0;
        for row in 0..h {
            for col in 0..w {
                let i = map[idx(row, col)];
                if i == 0 { continue; }
                for nrow in [row - 1, row, row + 1] {
                    if nrow < 0 || nrow >= h { continue; }
                    for ncol in [col - 1, col, col + 1] {
                        if ncol < 0 || ncol >= w { continue; }
                        let j = map[idx(nrow, ncol)];
                        if j == 0 { continue; }
                        if clusters[i] != clusters[j] {
                            for k in 1..nc {
                                if clusters[k] == clusters[j] {
                                    clusters[k] = clusters[i];
                                }
                            }
                            moved += 1;
                        }
                    }
                }
            }
        }
        if moved == 0 {
            break;
        }
    }

    let mut clusters = clusters.iter().collect::<HashSet<_>>();
    clusters.remove(&0);
    let r = clusters.len();

    println!("{r}");
}
