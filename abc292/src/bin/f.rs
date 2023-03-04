use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (a, b) = get_stdin_line().split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let (a, b) = (std::cmp::min(a, b), std::cmp::max(a, b));
    let (a, b) = (a as f64, b as f64);
    let r = b / a;

    let up_thr = (1f64 + 1. / 4.).sqrt();
    if r > up_thr {
        println!("{}", a * up_thr);
    } else {
    }
}
