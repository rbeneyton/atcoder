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
        k: usize,
        s: Chars,
    }
    debug_assert_eq!(s.len(), n);
    dur!(durations, "input done");


    // count letters
    let na = s.iter().filter(|x| **x == 'A').count();
    let nb = s.iter().filter(|x| **x == 'B').count();
    let nc = n - na - nb;
    // ordered
    let n1 = cmp::max(cmp::max(na, nb), nc);
    let n3 = cmp::min(cmp::min(na, nb), nc);
    let n2 = n - n1 - n3;

    enum SwapOp {
        S31, // 3<->1
        S32, // 3<->2
        S21, // 2<->1
    }
    let mut states = Vec::new();
    let mut res = 1u128; // initial string untouched
    // possible state, number of move possible for each case, number of cycles
    states.push((1u128, n3 as u128, n3 as u128, n2 as u128, 0u128));
    for _turn in 1..=k {
        let mut new_states = Vec::new();
        for (n, s31, s32, s21) in &states {
            let (n, s31, s32, s21) = (*n, *s31, *s32, *s21);
            if n == 0 { continue; }
            // S31
            new_states.push((n * s31,
                s31.saturating_sub(1), s32, s21));
            // S32
            new_states.push((n * s32,
                s31, s32.saturating_sub(1), s21));
            // S21
            new_states.push((n * s21,
                s31, s32, s21.saturating_sub(1)));
        }
        states.clear();
        std::mem::swap(&mut states, &mut new_states);

        let final_states : u128 = states.iter().map(|(x, _, _, _)| *x).sum();
        res += final_states;
        dbg!(&states, res);
    }

    println!("{res}");
}
