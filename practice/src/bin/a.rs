use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let mut lines = Vec::new();
    lines.push(get_stdin_line());
    lines.extend(get_stdin_line().split(' ')
                                 .map(|x| x.to_string())
                                 .collect::<Vec<_>>());
    let (a, b, c) = lines.iter()
                         .map(|x| x.parse::<u32>().unwrap())
                         .collect_tuple()
                         .unwrap();
    let s = get_stdin_line();

    println!("{} {}", a + b + c, s);
}
