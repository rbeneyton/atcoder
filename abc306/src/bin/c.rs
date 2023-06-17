// use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let mut counter = Vec::with_capacity(n);
    counter.resize(n,0u8);

    let line = get_stdin_line();
    for i in line
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
    {
        debug_assert!(i > 0);
        debug_assert!(i < n + 1);
        counter[i - 1] += 1;
        if counter[i - 1] == 2 {
            print!("{} ", i);
        }
    }
    println!("");
}
