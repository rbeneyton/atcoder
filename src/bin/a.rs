use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (N, K) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut A = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(A.len(), N);
    let mut start = 0;

    for _ in 0..K {
        start += 1;
        A.push(0); // TODO circular buffer
    }

    for i in start..(A.len()) {
        print!("{} ", A[i]);
    }
    println!("");
}
