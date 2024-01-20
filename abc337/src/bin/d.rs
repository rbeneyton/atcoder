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

const DURATIONS_SZ : usize = 64;
struct Durations {
    v : [(&'static str, std::time::Instant); DURATIONS_SZ],
    idx : usize,
}
impl Durations {
    fn new() -> Self {
        Self {
            v : [("start", std::time::Instant::now()); DURATIONS_SZ],
            idx : 1,
        }
    }
    fn push(&mut self, label: &'static str) {
        debug_assert!(self.idx < DURATIONS_SZ);
        self.v[self.idx] = (label, std::time::Instant::now());
        self.idx += 1;
    }
}
impl Drop for Durations {
    fn drop(&mut self) {
        self.push("end");
        let startup = self.v[0].1;
        let mut prev = None;
        for idx in 1..(self.idx) {
            let (what, timestamp) = self.v[idx];
            let duration = timestamp.saturating_duration_since(startup);
            if let Some(prev) = prev {
                let duration_from_prev = timestamp.saturating_duration_since(prev);
                eprintln!("{:>10.6} ({:>10.6}) {}",
                    duration.as_secs_f64(),
                    duration_from_prev.as_secs_f64(),
                    what);
            } else {
                eprintln!("{:>10.6}  {:>10}  {}", duration.as_secs_f64(), "", what);
            }
            prev = Some(timestamp);
        };
    }
}

macro_rules! durstart {
    ($name:ident) => (
        #[cfg(feature = "durations")]
        let mut $name = Durations::new();
    )
}

macro_rules! dur {
    ($name:ident, $label:literal) => (
        #[cfg(feature = "durations")]
        $name.push($label);
    )
}

pub fn get_stdin_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

// }}}

fn main() {
    durstart!(durations);
    input! {
        (h, w, k): (usize, usize, usize),
        s: [Chars; h],
    }
    dur!(durations, "input done");

    let map = s.iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x != '\n')
        .map(|x| match *x {
            '.' => 0u8,
            'o' => 1u8,
            'x' => 2u8,
            _ => panic!("invalid input"),
        })
        .collect::<Vec<_>>();
    let idx = |row, col| row * w + col;

    // goal: find minimum number of '.' in substring of len k without 'x'
    let mut min_by_row = usize::MAX;
    let col_start_end = std::cmp::max(
        (w as isize) - (k as isize) + 1,
        0
    ) as usize;
    // eprintln!("col_start_end={col_start_end}");
    let mut acc = Vec::with_capacity(w + 1);
    for row in 0..h {

        acc.clear();
        let (mut v0, mut v1, mut v2) = (0usize, 0usize, 0usize);
        acc.push((v0, v1, v2));
        for col in 0..w {
            match map[idx(row, col)] {
                0 => v0 += 1,
                1 => v1 += 1,
                2 => v2 += 1,
                _ => unreachable!(),
            }
            acc.push((v0, v1, v2));
        }
        for col in 0..col_start_end {

            // no subline with x
            // debug_assert_eq!(
            //     acc[col + k].2 - acc[col].2,
            //     map[idx(row, col)..idx(row, col + k)].iter().filter(|x| **x == 2).count()
            // );
            if acc[col + k].2 != acc[col].2 { continue; }
            // number of dot
            //
            // no subline with x
            let dots = acc[col + k].0 - acc[col].0;
            // debug_assert_eq!(
            //     dots,
            //     map[idx(row, col)..idx(row, col + k)].iter().filter(|x| **x == 0).count()
            // );
            min_by_row = std::cmp::min(min_by_row, dots);
        }
    }
    // dbg!(min_by_row);

    // transpose
    let mut mapt = Vec::new();
    mapt.resize(map.len(), 0u8);
    let idxt = |row, col| row * h + col;
    for row in 0..h {
        for col in 0..w {
            mapt[idxt(col, row)] = map[idx(row, col)];
        }
    }
    // immutable + same names + revert sizes
    let map = mapt;
    let (h, w) = (w, h);
    let idx = |row, col| row * w + col;
    // same logic on columns (which became rows now)
    let mut min_by_col = usize::MAX;
    let col_start_end = std::cmp::max(
        (w as isize) - (k as isize) + 1,
        0
    ) as usize;
    // eprintln!("col_start_end={col_start_end}");
    let mut acc = Vec::with_capacity(w + 1);
    for row in 0..h {
        acc.clear();
        let (mut v0, mut v1, mut v2) = (0usize, 0usize, 0usize);
        acc.push((v0, v1, v2));
        for col in 0..w {
            match map[idx(row, col)] {
                0 => v0 += 1,
                1 => v1 += 1,
                2 => v2 += 1,
                _ => unreachable!(),
            }
            acc.push((v0, v1, v2));
        }

        for col in 0..col_start_end {

            // no subline with x
            // debug_assert_eq!(
            //     acc[col + k].2 - acc[col].2,
            //     map[idx(row, col)..idx(row, col + k)].iter().filter(|x| **x == 2).count()
            // );
            if acc[col + k].2 != acc[col].2 { continue; }
            // number of dot
            //
            // no subline with x
            let dots = acc[col + k].0 - acc[col].0;
            // debug_assert_eq!(
            //     dots,
            //     map[idx(row, col)..idx(row, col + k)].iter().filter(|x| **x == 0).count()
            // );
            min_by_col = std::cmp::min(min_by_col, dots);
        }
    }
    // dbg!(min_by_col);

    let res = cmp::min(min_by_row, min_by_col);
    if res == usize::MAX {
        println!("-1");
    } else {
        println!("{res}");
    }

}
