use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (n, k) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut a = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(a.len(), n);
    let mut start = 0;

    for _ in 0..k {
        start += 1;
        a.push(0); // TODO circular buffer
    }

    for i in start..(a.len()) {
        print!("{} ", a[i]);
    }
    println!("");
}
