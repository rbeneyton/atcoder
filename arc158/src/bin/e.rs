use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (n, q) = get_stdin_line().split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let mut players = Vec::with_capacity(n);
    players.resize(n, 0u8);
    for _ in 0..q {
        let (c, x) = get_stdin_line().split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        debug_assert!(x > 0);
        debug_assert!(x - 1 < n);
        let p = &mut players[x - 1];
        debug_assert!(*p <= 3);
        match c {
            1 => *p += 1,
            2 => *p += 2,
            3 => println!("{}", if *p >= 2 { "Yes" } else { "No" }),
            _ => panic!(),
        }
    }
}
