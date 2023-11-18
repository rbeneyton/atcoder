// {{{ usual stuff

#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

use std::{
    collections::*,
    cmp,
    fmt,
    io::{self, Write},
    mem::swap,
};
use itertools::Itertools;
use proconio::input;
use proconio::marker::*;
use rustc_hash::{FxHashMap, FxHashSet};

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
        c: [Usize1; n],
        queries: [(Usize1, Usize1); q],
    }

    let mut colors = FxHashMap::default();
    for color in c.iter() {
        if !colors.contains_key(color) {
            colors.insert(color, colors.len());
        }
    }
    let n_color = colors.len();

    // use fixedbitset::FixedBitSet;
    // let mut boxs = Vec::with_capacity(n);
    // for color in c.iter() {
    //     let color = colors.get(color).expect("inconcistency");
    //     let mut e = FixedBitSet::with_capacity(n_color);
    //     e.set(*color, true);
    //     boxs.push(e);
    // }
    // drop(colors);
    //
    // for (a, b) in queries {
    //     let tmp = std::mem::take(&mut boxs[a]);
    //     boxs[b].union_with(&tmp);
    //     let res = boxs[b].count_ones(..);
    //
    //     println!("{res}");
    // }
    let mut boxs = Vec::with_capacity(n);
    for (idx, color) in c.iter().enumerate() {
        let color = *colors.get(color).expect("inconcistency");
        let mut e = BTreeSet::default();
        e.insert(color);
        boxs.push(e);
        debug_assert_eq!(boxs.len(), idx + 1);
    }
    drop(colors);

    for (a, b) in queries {
        let mut tmp = std::mem::take(&mut boxs[a]);
        if tmp.len() > boxs[b].len() {
            std::mem::swap(&mut tmp, &mut boxs[b]);
        }
        for color in tmp {
            boxs[b].insert(color);
        }
        let res = boxs[b].len();
        println!("{res}");
    }
}
