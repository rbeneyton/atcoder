use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let _n = get_stdin_line().parse::<usize>().unwrap();
    let s = get_stdin_line();
    for (c1, c2) in s.chars().tuple_windows() {
        if c1 == c2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
