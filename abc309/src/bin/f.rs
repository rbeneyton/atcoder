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

fn solve() {
}

#[test]
fn test_solve()
{
}

// #[proconio::fastout]
fn main() {
    input! {
        n: usize,
        // b: [[usize; 3]; n], // proconio vec's
    }
    let mut bo = Vec::<[u32; 3]>::with_capacity(n);
    for _ in 0..n {
        input! {
            a: u32,
            b: u32,
            c: u32,
        }
        bo.push([a, b, c]);
    }
    let bo = bo;

    // get 1st & 2nd higher values on each axis
    let mut maxs = [0u32; 3];
    let mut max2s = [0u32; 3];
    let mut heap = BinaryHeap::new();
    heap.reserve(n);
    for idx in 0..3 {
        for v in bo.iter().map(|x| x[idx]) {
            heap.push(v);
        }
        maxs[idx] = heap.pop().unwrap();
        max2s[idx] = heap.pop().unwrap();
        heap.clear();
    }
    drop(heap);
    let maxs = maxs;
    let max2s = max2s;
    dbg!(&maxs, &max2s);

    let mut sorted_maxs = maxs.clone();
    sorted_maxs.sort();
    let sorted_maxs = sorted_maxs;
    let mut sorted_max2s = max2s.clone();
    sorted_max2s.sort();
    let sorted_max2s = sorted_max2s;
    dbg!(&sorted_maxs, &sorted_max2s);

    for i in 0..n {
        let mut bi = bo[i].clone();
        bi.sort();
        dbg!(&bi, &sorted_max2s);
        if bi < sorted_max2s { continue; }
        dbg!(i);
        println!("Yes");
        return;
    }
    println!("No");
}
