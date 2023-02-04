use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (_n, m) = get_stdin_line().split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    for _ in 0..m {
    }
}
