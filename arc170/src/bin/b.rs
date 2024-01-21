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
        n: usize,
        a: [i8; n],
    }
    dur!(durations, "input done");

    // vector of index where equality occured
    const DIFF : usize = 18;
    const DIFF_OFF : i8 = 9;
    let idx = |i, diff| i * DIFF + diff;

    let mut diffs = Vec::new();
    diffs.resize(DIFF * n, usize::MAX);

    // all diffs for all combinaison (store index where diff occurs)
    'scan: for i in 0..n {
        let mut all_diffs = DIFF;
        for j in (i+1)..n {
            let diff = (a[j] - a[i] + DIFF_OFF) as usize;
            if diffs[idx(i, diff)] == usize::MAX {
                diffs[idx(i, diff)] = j;
                all_diffs -= 1;
                if all_diffs == 0 { continue 'scan; }
            }
        }
    }
    // ffill
    for i in 1..n {
        for diff in 0..DIFF {
            if diffs[idx(i, diff)] == usize::MAX {
                diffs[idx(i, diff)] = diffs[idx(i - 1, diff)];
            }
        }
    }

    eprintln!("diffs:{}", itertools::join((0..DIFF).map(|x| format!("{:3}", x as i8 - DIFF_OFF)), ","));
    for i in 0..n {
        eprintln!("{i:5}:{}", itertools::join(
            (0..DIFF)
                .map(|x| format!("{:3}", diffs[idx(i, x)])), ","))
    }

    // ffill
    let mut diff2s = Vec::new();
    diff2s.resize(DIFF * n, 0); // TODO bitset

    for i in 1..n {
        for d in 0..DIFF {
            if diffs[idx(i, d)] == 0 {
                diff2s[idx(i, d)] = diff2s[idx(i-1, d)];
            }
        }
    }
    eprintln!("diff2s:{}", itertools::join((0..DIFF).map(|x| format!("{:3}", x as i8 - DIFF_OFF)), ","));
    for i in 0..n {
        eprintln!("{i:6}:{}", itertools::join(
            (0..DIFF)
                .map(|x| format!("{:3}", diff2s[idx(i, x)])), ","))
    }

    // find equalities
    let mut good = 0;
    for l in 0..n {
        'scanr: for r in (l+3)..n {
            for diff in 0..DIFF {
                if diffs[idx(r, diff)] >= diffs[idx(l, diff)] + 2 {
                    good += 1;
                    continue 'scanr;
                }
            }
        }
    }

    println!("{good}");
}
