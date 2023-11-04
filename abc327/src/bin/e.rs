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
        p: [u32; n],
    }
    let f = 0.9f32;

    let mut powers = Vec::with_capacity(n);
    let mut powers_sum = Vec::with_capacity(n);
    let (mut tmp, mut sum) = (1f32, 1f32);
    for _ in 0..n {
        powers.push(tmp);
        powers_sum.push(1. / sum);
        tmp *= f;
        sum += tmp;
    }
    let powers = powers;

    let penatly = (0..=n)
        .map(|x| if x > 0 { 1200f32 / (x as f32).sqrt() } else { 0. })
        .collect::<Vec<_>>();

    let score = |q: &[bool]| -> f32 {
        debug_assert_eq!(q.len(), n);
        let k = q.iter().filter(|x| **x).count();
        if k == 0 {
            return -1e9;
        }
        let mut num = 0f32;
        let mut seen = 0;
        for i in (0..n).filter(|x| q[*x]) {
            seen += 1;
            num += (p[i] as f32) * powers[k - seen];
        }
        debug_assert_eq!(seen, k);
        num * powers_sum[k - 1] - penatly[k]
    };
    dur!(durations, "pre computations done");

    // sort by ascending score, latest win
    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by(|a, b| p[*a].cmp(&p[*b]).then_with(|| a.cmp(&b)));
    let order = order;
    // dbg!(&order);

    // initial k-scan
    let mut q = Vec::with_capacity(n);
    q.resize(n, true);
    let mut best_q = q.clone();
    let mut s = score(&q);
    // dbg!(&s);
    for k in 0..(n - 1) {
        q[order[k]] = false;
        let s2 = score(&q);
        if s2 > s {
            best_q = q.clone();
            s = s2;
        }
    }

    // marginal move scan
    q = best_q;
    // dbg!(&s);
    loop {
        let mut modified = false;
        for k in (0..n).rev() {
            // if !q[k] && k + 5 < n && p[k] < s - 100f32 { continue; }
            q[k] = !q[k];
            let s2 = score(&q);
            if s2 < s {
                q[k] = !q[k];
            } else {
                s = s2;
                modified = true
            }
        }
        if !modified {
            break;
        }
    }
    dur!(durations, "main computations done");

    println!("{:.15}", s);
}
