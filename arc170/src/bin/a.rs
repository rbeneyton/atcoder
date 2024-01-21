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
        s: Chars,
        t: Chars,
    }
    dur!(durations, "input done");

    debug_assert_eq!(s.len(), n);
    debug_assert_eq!(t.len(), n);

    // memory locality
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push((s[i], t[i]));
    }

    // simplify v
    let (mut a_seen, mut b_seen) = (0, 0);
    let mut v2 = Vec::with_capacity(n);
    for i in 0..n {
        if v[i].0 != v[i].1 || a_seen <= 2 || b_seen <= 2 {
            v2.push(v[i]);
            (a_seen, b_seen) = (0, 0);
        } else {
            a_seen += (v[i].0 == 'A') as usize;
            b_seen += (v[i].0 == 'B') as usize;
        }
    }
    let mut v = v2;
    let n = v.len();

    let mut swap = 0;
    let mut first_immutable_a = (0..n)
        .filter(|x| v[*x].0 == v[*x].1 && v[*x].0 == 'A')
        .next();
    let last_immutable_b = (0..n)
        .rev()
        .filter(|x| v[*x].0 == v[*x].1 && v[*x].0 == 'B')
        .next();
    'scan: for i in 0..n {
        if v[i].0 != v[i].1 {
            swap += 1;
            if v[i].0 == 'B' {
                v[i].0 = 'A';
                // optimal double change
                for j in (i+1)..n {
                    if v[j].0 != v[j].1 {
                        if v[j].0 == 'A' {
                            v[j].0 = 'B';
                            continue 'scan;
                        }
                    }
                }
                // at least one possibility where B stays B
                if let Some(idx) = last_immutable_b {
                    if idx > i {
                        continue 'scan;
                    }
                }
                // fallback to mutable
                for j in (i+1)..n {
                    if v[j].0 == 'B' {
                        continue 'scan;
                    }
                }
                // fail: no 2change allowed
            } else {
                v[i].0 = 'B';
                // first incorrect case reversed, need previous letter 'A'
                if let Some(idx) = first_immutable_a {
                    if idx < i {
                        continue 'scan;
                    }
                }
                // fallback to mutable
                for j in 0..i {
                    if v[j].0 == 'A' {
                        first_immutable_a = Some(j);
                        continue 'scan;
                    }
                }
                // fail no previous 'A' to allow 2change
            }
            println!("-1");
            return;
        }
    }

    println!("{swap}");
}
