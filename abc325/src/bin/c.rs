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
    let mut nodes = Vec::new();
    for row in 0..h {
        input! {
            line: Chars,
        }
        debug_assert_eq!(line.len(), w as usize);
        for col in 0..w {
            if line[col as usize] == '#' {
                nodes.push((row, col));
            }
        }
    }
    let n = nodes.len();

    // compute disance matrix triangle
    let mut distances = Vec::new();
    distances.resize(n * n, false);
    let idx = |i, j| i * n + j;
    for i in 0..n {
        let ii = &nodes[i];
        for j in (i + 1)..n {
            let jj = &nodes[j];
            distances[idx(i, j)] = std::cmp::max((ii.0 - jj.0).abs(), (ii.1 - jj.1).abs()) <= 1;
        }
    }
    drop(nodes);

    let mut clusters = (0..n).collect::<Vec<_>>();
    'scan: loop {
        // eprintln!("{}", itertools::join(clusters.iter().map(ToString::to_string), " "));
        for i in 0..n {
            for j in (i + 1)..n {
                if clusters[j] != clusters[i] && distances[idx(i, j)] {
                    let clu = std::cmp::min(clusters[j], clusters[i]);
                    // eprintln!("{j}:{}→{} because {i}", clusters[j], clu);
                    for k in 0..n {
                        if clusters[k] == clusters[i] || clusters[k] == clusters[j] {
                            clusters[k] = clu;
                        }
                    }
                    continue 'scan;
                }
            }
        }
        break;
    }

    let clusters = clusters.iter().collect::<HashSet<_>>();
    let r = clusters.len();

    println!("{r}");
}
