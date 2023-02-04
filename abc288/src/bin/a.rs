use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    for _ in 0..n {
        let (a, b) = get_stdin_line().split(' ')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        println!("{}", a + b);
    }
}
