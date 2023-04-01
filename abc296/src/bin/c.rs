use itertools::Itertools;
use rustc_hash::FxHashSet;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (_n, x) = get_stdin_line().split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut a = FxHashSet::default();
    for v in get_stdin_line()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
    {
        a.insert(v);
    }
    for v in &a {
        if a.contains(&(v - x)) || a.contains(&(v + x)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
