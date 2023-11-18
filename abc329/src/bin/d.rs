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
        n: usize, // candidates
        m: usize, // number of votes
        a: [u32; m],
    }

    let mut wins = Vec::new(); // number of win per candidates
    wins.resize(n, 0u32);

    // {{{ Cand

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct Cand { idx: usize, vote: u32 }
    impl Ord for Cand {
        fn cmp(&self, other: &Cand) -> cmp::Ordering {
            self.vote.cmp(&other.vote).then_with(|| other.idx.cmp(&self.idx))
        }
    }
    impl PartialOrd for Cand {
        fn partial_cmp(&self, other: &Cand) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    // }}}

    let mut heap = BinaryHeap::new();
    heap.reserve(n);

    for winner in a {
        debug_assert!(winner > 0);
        let winner = winner as usize - 1;
        debug_assert!(winner < n);

        wins[winner] += 1;
        heap.push(Cand {
            idx : winner,
            vote: wins[winner],
        });
        let current_winner = heap.peek().unwrap().idx + 1;

        println!("{current_winner}");
    }
}
