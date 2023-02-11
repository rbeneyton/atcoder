use itertools::Itertools;
use std::collections::BTreeSet;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (n, m) = get_stdin_line().split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let n = (n + 1) as u32;
    if m == 0 {
        for i in 1..n {
            print!("{} ", i);
        }
        println!("");
        return;
    }
    let mut a = BTreeSet::default();
    for v in get_stdin_line()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
    {
        a.insert(v);
        debug_assert!(v < n);
    }
    debug_assert_eq!(a.len(), m);

    // XX pop_first unusable
    while let Some(a_min) = a.iter().next() {
        let a_min = *a_min;
        a.remove(&a_min);
        let mut v = vec![a_min];
        for ai in (a_min + 1)..n {
            v.push(ai);
            if a.contains(&ai) {
                a.remove(&ai);
                continue;
            } else {
                break;
            }
        }
        for vi in v.iter().rev() {
            print!("{} ", *vi);
        }
        let v_last = v.last().unwrap() + 1;
        let v_until = match a.iter().next() {
            Some(v) => *v,
            None => n,
        };
        for i in v_last..v_until {
            print!("{} ", i);
        }
    }
    println!("");
}
