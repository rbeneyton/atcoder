use itertools::Itertools;
use rustc_hash::FxHashSet;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (n, s) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut ss = Vec::new();
    for _ in 0..n {
        let (a, b, c) = get_stdin_line()
            .chars()
            .skip(3)
            .map(|x| x.to_digit(10).unwrap())
            .collect_tuple()
            .unwrap();
        ss.push(a * 100 + b * 10 + c);
    }
    let mut ts = FxHashSet::default();
    for _ in 0..s {
        let (a, b, c) = get_stdin_line()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect_tuple()
            .unwrap();
        ts.insert(a * 100 + b * 10 + c);
    }
    let res = ss.iter()
        .filter(|x| ts.contains(x))
        .count();

    println!("{}", res);
}
