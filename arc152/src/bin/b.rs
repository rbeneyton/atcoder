#![allow(non_upper_case_globals)]
#![allow(unused_macros)]

use itertools::Itertools;
use std::io::prelude::*;
use std::fmt;
use std::cmp;
// use std::collections::BTreeMap;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

#[allow(dead_code)]
#[allow(unused_variables)]
const LOG_LVL : u8 = 100;

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
#[macro_export]
macro_rules! logcont {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
#[macro_export]
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
            std::io::stderr().flush().unwrap();
        }
    })
}

fn main() {
    let (n, l) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let a = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(a.len(), n);
    for i in &a {
        debug_assert!(*i < l);
    }
    logln!(10, "n:{} l:{} a:{}", n, l, itertools::join(a.iter(), ","));

    // 1 - find extremity node closer to its end
    let min_east = l - a[n - 1] <= a[0];

    dbg!(min_east);
    // 2 - reorder vector if not to_east
    dbg!(&a);
    let a = if min_east {
        a
    } else {
        a.iter().rev().map(|x| l - *x).collect()
    };
    dbg!(&a);
    debug_assert!(l - a[n - 1] <= a[0]);

    // 3 - get final step
    let last_step = l - a[n - 1];
    debug_assert!(last_step > 0); // no node at ends
    logln!(10, "min_east:{} last_step:{}", min_east, last_step);

    // 2 - find 2 node with distance up or greated than last_step
    let (start_idx, start_dist) = (0..(n - 1))
        .map(|idx| (idx, a[idx + 1] - a[idx]))
        .filter(|(_, dist)| *dist >= last_step)
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    logln!(10, "start_idx:{} start_dist:{}", start_idx, start_dist);

    // 3 - mirror a to 3 times
    let mut a = a;
    a.resize(3 * a.len(), 0);
    for i in 0..n {
        a[n + i] = l + (l - a[n - 1 - i]);
        a[2 * n + i] = 2 * l + a[i];
    }
    let a = a;
    // 3 - compute path

    // same direction
    let time_same_direction = {
        // W--A-B--E
        //    S S+
        let (mut time_a, mut time_b) = (0, 0);
        // A start at start_idx until last_step
        time_a += a[n - 1] - a[start_idx];
        // B start at start_idx+1 until last_step with round to east end
        time_b += a[n] - a[start_idx + 1];
        // A&B wait each other at last end
        let time = cmp::max(time_a, time_b);
        logln!(50, "first wait: A:{} B:{} =>{}", time_a, time_b, time);
        time_a = time;
        time_b = time;

        // W--B-A--E
        // A goes east corner then first
        assert_eq!(a[2 * n - 1] - a[n - 1], last_step + l - a[0]);
        time_a += a[2 * n - 1] - a[n - 1];
        // B goes west corner then first
        assert_eq!(l - last_step + a[0], a[2 * n] - a[n]);
        time_b += a[2 * n] - a[n];
        // A&B wait each other at first node
        let time = cmp::max(time_a, time_b);
        logln!(50, "second wait: A:{} B:{} =>{}", time_a, time_b, time);
        time_a = time;
        time_b = time;

        // W--A-B--E
        // A goes west corner then goes back to its start position
        assert_eq!(a[0] + a[start_idx], a[2 * n + start_idx] - a[2 * n - 1]);
        time_a += a[2 * n + start_idx] - a[2 * n - 1];
        // B goes back to its start position
        assert_eq!(a[start_idx + 1] - a[0], a[2 * n + start_idx + 1] - a[2 * n]);
        time_b += a[2 * n + start_idx + 1] - a[2 * n];

        let time = cmp::max(time_a, time_b);
        logln!(50, "finish: A:{} B:{} =>{}", time_a, time_b, time);

        time
    };

    let time_opposite_direction = {
        // let walk = |start_pos| {
        //     let (mut time_a, mut time_b) = (0, 0);
        //     let (mut pos_a, mut pos_b) = (start_pos, start_pos + 1);
        //     loop {
        //         let step_a = 
        //     }
        // };
        // for start in 0..(l - 1) {
        // }
        10 * l
    };

    println!("{}", cmp::min(time_same_direction, time_opposite_direction));
}
