// use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .next()
        .unwrap();
    let s = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(s.len(), n);
    
    print!("{} ", s[0]);
    for i in 1..n {
        print!("{} ", s[i] - s[i - 1]);
    }
    println!("");
}
