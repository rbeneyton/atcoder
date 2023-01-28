use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (h, w) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut res = 0;
    for _ in 0..h {
        let line = get_stdin_line();
        debug_assert_eq!(line.len(), w);
        for c in line.chars() {
            if c == '#' {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
