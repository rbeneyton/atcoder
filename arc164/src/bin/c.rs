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

#[derive(Default, Clone)]
pub struct Node {
    va: Vec<u32>,
    vb: Vec<u32>,
    score_bob: u32,
    max_score_bob: u32,
}
pub type Nodes = Vec<Node>;
impl Node {
    pub fn new(va: Vec<u32>, vb: Vec<u32>) -> Self {
        Self {
            va,
            vb,
            ..Default::default()
        }
    }
    pub fn play_alice(&mut self) {
        for idx in 0..self.va.len() {
            swap(&mut self.va[idx], &mut self.vb[idx]);
            let score_bob = self.score_bob;

            self.play_bob();

            // restore state
            swap(&mut self.vb[idx], &mut self.va[idx]);
            self.score_bob = score_bob;
        }
    }
    pub fn play_bob(&mut self) {
        for idx in 0..self.va.len() {
            let score = self.score_bob;
            self.score_bob += self.va[idx];
            self.max_score_bob = std::cmp::max(self.max_score_bob, self.score_bob);
            let va_bckp = self.va[idx];
            let vb_bckp = self.vb[idx];
            self.va.swap_remove(idx);
            self.vb.swap_remove(idx);

            self.play_alice();

            // restore state
            self.va.push(va_bckp);
            self.vb.push(vb_bckp);
            let n = self.va.len() - 1;
            debug_assert_eq!(self.va.len(), self.vb.len());
            if idx != n {
                self.va.swap(idx, n);
                self.vb.swap(idx, n);
            }

            self.score_bob = score;
        }
    }
}

fn solve_c(va : Vec<u32>, vb : Vec<u32>) -> u32 {
    let mut root = Node::new(va, vb);

    root.play_alice();
    root.max_score_bob
}

#[test]
fn test_solve_c()
{
    assert_eq!(solve_c(3, vec![6, 2, 5], vec![4, 1, 3]), 12);

    assert_eq!(solve_c(5,
        vec![
            166971716,
            219878198,
            918378176,
            610749017,
            701849287,
        ],
        vec![
            552987438,
            619875818,
            518975015,
            285601372,
            307601390,
        ]),
        3078692091);
}

// #[proconio::fastout]
fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for _ in 0..n {
        let (ai, bi) = get_stdin_line().split(' ')
            .map(|x| x.to_string())
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        a.push(ai);
        b.push(bi);
    }
    println!("{}", solve_c(a, b));
}

