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
        h: usize, // h rows
        w: usize, // w columns
    }
    // add borders
    let h_ext = h + 2;
    let w_ext = w + 2;

    let n = h_ext * w_ext;
    let mut g = Vec::new();
    g.resize(n, 0u32);
    let idx = |row, col| {
        debug_assert!(row < h_ext);
        debug_assert!(col < w_ext);
        row * w_ext + col
    };
    let mut id = 1;
    for row in 0..h {
        input! {
            line: Chars,
        }
        debug_assert_eq!(line.len(), w);
        for col in 0..w {
            if line[col] == '#' {
                g[idx(row + 1, col + 1)] = id;
                id += 1;
            }
        }
    }

    // dbg!(&id);

    // interacting coalescing
    let mut tmp = Vec::new();
    'scan: loop {
        let mut modified = 0;
        for row in 1..=h {
            for col in 1..=w {
                let id = g[idx(row, col)];
                if id == 0 { continue; }
                tmp.clear();

                for nrow in [row - 1, row, row + 1] {
                    for ncol in [col - 1, col, col + 1] {
                        let did = g[idx(nrow, ncol)];
                        if did == 0 { continue; }
                        tmp.push((nrow, ncol));
                    }
                }
                if tmp.len() > 1 {
                    let id_min = tmp.iter().map(|(nrow, ncol)| g[idx(*nrow, *ncol)]).min().unwrap();
                    let id_max = tmp.iter().map(|(nrow, ncol)| g[idx(*nrow, *ncol)]).max().unwrap();
                    if id_min != id_max {
                        for (row, col) in &tmp {
                            g[idx(*row, *col)] = id_min;
                        }
                        modified += 1;
                    }
                }
            }
        }
        if modified == 0 {
            break;
        }
    }

    if false {
        for row in 0..h_ext {
            for col in 0..w_ext {
                let id = g[idx(row, col)];
                let id = if id > 0 { format!("{:>5}", id) } else { format!("    .") };
                print!("{id}");
            }
            println!("");
        }
    }
    let ids = g.iter().collect::<std::collections::HashSet<_>>();
    debug_assert!(ids.contains(&0));
    let r = ids.len() - 1;

    println!("{r}");
}
