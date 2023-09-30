// {{{ usual stuff

#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]
#![allow(non_snake_case)]

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

// #[proconio::fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        s: Chars,
    }
    debug_assert_eq!(s.len(), n);
    let s : Vec<bool> = s.iter().map(|c| *c == '1').collect();
    dbg!(&s);
    let mut ss = BTreeMap::new(); // key: idx, value: s
    {
        ss.insert(1usize, s[0]);
        for (idx, (a, b)) in s.iter().tuple_windows().enumerate() {
            if a != b {
                ss.insert(idx + 1 + 1, *b);
            }
        }
        dbg!(&ss);
    }
    for _ in 0..q {
        input! {
            (c, L, R): (u8, usize, usize),
        }
        debug_assert!(L >= 1);
        debug_assert!(L <= n);
        debug_assert!(R >= 1);
        debug_assert!(R <= n);
        let L = L;
        let R = R;
        match c {
            1 => {
                dbg!("action 1", L, R, &ss);
                let left = *ss.range(..=L).next_back().unwrap().0;
                if left != L {
                    ss.insert(L, ss[&left]);
                }
                let right = *ss.range(..=R).next_back().unwrap().0;
                if right != R {
                    ss.insert(R, ss[&right]);
                }
                //dbg!("expand", L, R);
                //dbg!(&ss);
                // invert
                for (_, v) in ss.range_mut(L..=R) {
                    *v = !*v;
                }
                //dbg!("flip", L, R);
                //dbg!(&ss);
                let left = *ss.range((L+1)..).next().unwrap_or((&0, &true)).0;
                if left != 0 && ss.contains_key(&L) && ss[&left] == ss[&L] {
                    ss.remove(&left);
                }
                let left = *ss.range(..L).next_back().unwrap_or((&0, &true)).0;
                if left != 0 && ss.contains_key(&L) && ss[&left] == ss[&L] {
                    ss.remove(&L);
                }
                let right = *ss.range((R+1)..).next().unwrap_or((&0, &true)).0;
                if right != 0 && ss.contains_key(&R) && ss[&right] == ss[&R] {
                    ss.remove(&right);
                }
                let right = *ss.range(..R).next_back().unwrap_or((&0, &true)).0;
                if right != 0 && ss.contains_key(&R) && ss[&right] == ss[&R] {
                    ss.remove(&R);
                }
                dbg!("after", &ss);
            },
            2 => {

                // let mut n1 : usize = 0;
                // let mut max_n1 : usize = 0;
                // for idx in L..=R {
                //     if s[idx] {
                //         n1 += 1;
                //         max_n1 = std::cmp::max(max_n1, n1);
                //     } else {
                //         n1 = 0;
                //     }
                // }
                // println!("{}", max_n1);
                let mut max_n1 : usize = 0;
                for ((kl, vl), (kr, vr)) in ss.tuple_windows() {
                    if *vl {
                        debug_assert_eq!(*vr, false);
                        let n1 = std::cmp::max(*kr, R) - kl + 1;
                        max_n1 = std::cmp::max(max_n1, n1);
                    }
                    if *kr >= R {
                        break;
                    }
                }
                dbg!("action 2", L, R, &ss, max_n1);
                println!("{}", max_n1);
            },
            _ => panic!(),
        }
    }
}
