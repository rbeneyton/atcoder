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

fn feasable(a: &[usize], k: usize) -> bool {
    //eprintln!("feasable({}, {})", itertools::join(a.iter().map(|x| format!("{x}")), ","), k);
    let n = 2 * k - 1;
    //eprintln!("len:{}", n);
    if a.len() < n {
        return false;
    }
    'start: for start in 0..=(a.len() - n) {
        //eprintln!("start:{}, threshold:{}", start,
        //    itertools::join((0..n)
        //        .map(|x| if x < k { x + 1 } else { n - x })
        //        .map(|x| format!("{x}")), ","));
        for (idx, ai) in a[start..(start+n)].iter().enumerate() {
            let min = if idx < k { idx + 1 } else { n - idx };
            if *ai < min {
                //eprintln!("nope: start:{} idx:{} ai:{}<min:{}", start, idx, *ai, min);
                continue 'start;
            }
        }
        return true;
    }
    false
}

#[test]
fn test_d_feasable()
{
    assert_eq!(feasable(&[1, ], 1), true);
    assert_eq!(feasable(&[1, 1, ], 2), false);
    assert_eq!(feasable(&[1, 1, 1], 2), false);
    assert_eq!(feasable(&[1, 2, 1], 2), true);
}

fn feasable2(a: &[usize], k: usize) -> bool {
    //eprintln!("feasable2({}, {})", itertools::join(a.iter().map(|x| format!("{x}")), ","), k);
    let n = 2 * k - 1;
    //eprintln!("len:{}", n);
    if a.len() < n {
        return false;
    }
    'top: for idx in (k-1)..(a.len() - k + 1) {
        //dbg!(idx, a[idx]);
        if a[idx] >= k {
            //dbg!("test idx");
            // up side
            for i in 1..k {
                if a[idx - i] < k - i {
                    continue 'top;
                }
            }
            // down side
            for i in 1..k {
                if a[idx + i] < k - i {
                    continue 'top;
                }
            }
            return true;
        }
    }
    false
}

#[test]
fn test_d_feasable2()
{
    assert_eq!(feasable2(&[1, ], 1), true);
    assert_eq!(feasable2(&[1, 1, ], 2), false);
    assert_eq!(feasable2(&[1, 1, 1], 2), false);
    assert_eq!(feasable2(&[1, 2, 1], 2), true);
}


fn main() {
    durstart!(durations);
    input! {
        n: usize,
        a: [usize; n],
    }
    dur!(durations, "input done");

    let max_k = std::cmp::min(
        (n + 1) / 2,
        *a.iter().max().unwrap()
    );
    for k in (1..=max_k).rev() {
        if feasable2(&a, k) {
            println!("{}", k);
            break;
        }
    }
}
