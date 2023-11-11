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

fn main() {
    input! {
        mut s: String,
    }
    let mut res = Vec::with_capacity(s.len());

    for c in s.chars() {
        res.push(c);
        if res.len() >= 3 && res[res.len() - 3] == 'A' && res[res.len() - 2] == 'B' && res[res.len() - 1] == 'C' {
            res.truncate(res.len() - 3);
        }
    }
    let res: String = res.into_iter().collect();
    println!("{res}");
}

    // input! {
    //     mut s: Bytes,
    // }
    // let mut tmp = Vec::with_capacity(s.len());
    // let A = 'A' as u8;
    // let B = 'B' as u8;
    // let C = 'C' as u8;
    // //let mut stderr: std::io::Stderr = std::io::stderr();
    // loop {
    //     tmp.clear();
    //     let n = s.len();
    //     let mut i = 0;
    //     let mut modified = false;
    //     //eprintln!("{}", itertools::join(s.iter().map(|x| format!("{}", x)), ""));
    //     // stderr.write(&s);
    //     // stderr.write(b"\n");
    //     loop {
    //         if i >= n {
    //             break;
    //         }
    //         if i + 2 < n && s[i] == A && s[i+1] == B && s[i+2] == C {
    //             modified = true;
    //             i += 3;
    //         } else {
    //             tmp.push(s[i]);
    //             i += 1;
    //         }
    //     }
    //     if !modified {
    //         break;
    //     }
    //     std::mem::swap(&mut s, &mut tmp);
    // }
    // // eprintln!("res:");
    // let mut stdout: std::io::Stdout = std::io::stdout();
    // stdout.write(&s);
    // stdout.write(b"\n");
    // stdout.flush();


    // loop {
    //     let mut drop_idx = None;
    //     for (a, b, c) in s.iter().enumerate().filter(|(_, x)| **x != ' ').tuple_windows() {
    //         if *a.1 == 'A' && *b.1 == 'B' && *c.1 == 'C' {
    //             drop_idx = Some((a.0, b.0, c.0));
    //             break;
    //         }
    //     }
    //     if let Some((a, b, c)) = drop_idx {
    //         s[a] = ' ';
    //         s[b] = ' ';
    //         s[c] = ' ';
    //     } else {
    //         break;
    //     }
    // }
    // for c in s {
    //     if c != ' ' {
    //         print!("{c}");
    //     }
    // }
