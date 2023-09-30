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
    let mut ss = BTreeMap::new(); // key:start_idx, value:(len, what)
    {
        let (mut start, mut len) = (0, 1);
        for (idx, (a, b)) in s.iter().tuple_windows().enumerate() {
            if a == b {
                len += 1;
            } else {
                ss.insert(start, (len, *a));
                (start, len) = (idx + 1, 1);
            }
        }
        ss.insert(start, (len, s[n - 1]));
        dbg!(&ss);
        // for (a, b) in ss.iter().tuple_windows() {
        //     debug_assert!(a.0 + a.1 == b.0);
        //     debug_assert!(a.2 != b.2);
        // }
    }
    for _ in 0..q {
        input! {
            (c, L, R): (u8, usize, usize),
        }
        debug_assert!(L >= 1);
        debug_assert!(L <= n);
        debug_assert!(R >= 1);
        debug_assert!(R <= n);
        let L = L - 1;
        let R = R - 1;
        match c {
            1 => {
                let left = ss.range(..=L).next_back().unwrap();
                let left = *left.0;
                if left != L {
                    // split
                    ss.insert(L, ss[&left]);
                    ss.get_mut(&left).unwrap().0 = L - left;
                    ss.get_mut(&L).unwrap().0 -= L - left;
                }
                let right = ss.range(R..).next().unwrap();
                let right = *right.0;
                if right != R {
                    // split
                    ss.insert(R, ss[&right]);
                    ss.get_mut(&R).unwrap().0 = right - R;
                    ss.get_mut(&right).unwrap().0 -= right - R;
                }
                // invert
                for (_, ref mut v) in ss.range(L..=R) {
                    v.1 = !v.1;
                }

                // for e in ss {
                //     if e.0 < L { continue; }
                //     if e.0 == L {
                //         if e.1 < R - L {
                //         }
                //         break;
                //     }

                // for idx in L..=R {
                //     s[idx] = !s[idx];
                // }
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
            },
            _ => panic!(),
        }
    }
}
