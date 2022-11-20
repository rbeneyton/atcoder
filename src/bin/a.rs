use itertools::Itertools;
use std::io::prelude::*;
use std::fmt;
use std::cmp;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

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
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(a.len(), n);

    // seats
    let mut s = Vec::new();
    s.resize(l, false);

    let mut latest_free = 0;
    let mut first_free = 0;

    let mut feasable = true;

    logln!(10,"{} group fo {} seats", n, l);

    'scan: for i in 0..n {
        logstart!(60,"{}: ", i);
        for si in &s { logcont!(60, "{} ", si); }
        logstop!(60, "");
        logln!(10, "{}:a[{}]={}", i, i, a[i]);
        logln!(10, "first_free:{} latest_free:{}", first_free, latest_free);

        // find worst place
        if a[i] == 1 {
            // build a hole of 1
            let worst = latest_free + 1;
            if worst < l {
                debug_assert_eq!(s[worst], false);
                s[worst] = true;
                latest_free = worst + 1;
                debug_assert!(latest_free >= l || s[latest_free] == false);
                logln!(10, "in worst place {}", worst);
            } else {
                // first free
                for free in first_free..l {
                    if s[free] == false {
                        s[free] = true;
                        s[free] = true;
                        if free == first_free {
                            first_free = free + 1;
                        }
                        latest_free = cmp::max(latest_free, free + 1);
                        continue 'scan;
                    }
                }
                logln!(10, "fail due to {}:a[{}]={} place worst:{} latest_free:{}", i, i, a[i], worst, latest_free);
                feasable = false;
                break 'scan;
            }
        }
        if a[i] == 2 {
            // build a hole of 1
            let worst = latest_free + 1;
            if worst < l - 1 {
                debug_assert_eq!(s[worst], false);
                debug_assert_eq!(s[worst + 1], false);
                s[worst] = true;
                s[worst + 1] = true;
                latest_free = worst + 2;
                debug_assert!(latest_free >= l || s[latest_free] == false);
                logln!(10, "in worst place {}+{}", worst, worst + 1);
            } else {
                // first free place of 2
                for free in first_free..(l - 1) {
                    if s[free] == false && s[free + 1] == false {
                        s[free] = true;
                        s[free + 1] = true;
                        if free == first_free {
                            first_free = free + 1;
                        }
                        latest_free = cmp::max(latest_free, free + 2);
                        continue 'scan;
                    }
                }
                logln!(10, "fail due to {}:a[{}]={} place worst:{} latest_free:{}", i, i, a[i], worst, latest_free);
                feasable = false;
                break 'scan;
            }
        }
    }
    println!("{}", if feasable { "Yes" } else { "No" });
}
