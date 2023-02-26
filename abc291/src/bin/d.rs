use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for _ in 0..n {
        let (ai, bi) = get_stdin_line().split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        a.push(ai);
        b.push(bi);
    }

    let mut res = 2; // size == 1 => one flip possible
    for idx in 1..n {
        // do we double the current possible flip or not?
        let aa = a[idx] == a[idx - 1];
        let ab = a[idx] == b[idx - 1];
        let bb = b[idx] == b[idx - 1];
        let ba = b[idx] == a[idx - 1];

        let mut flip = true;
        let mut force = false;

        match (aa, ab) {
            (true, true) => flip = false, // invalidate a[idx - 1],
            (true, false) => force = true, // force flip,
            (false, true) => force = true, // forbid flip,
            (false, false) => (), // allow flip or not,
        }
        match (bb, ba) {
            (true, true) => flip = false, // invalidate b[idx - 1],
            (true, false) => force = true, // force flip,
            (false, true) => force = true, // forbid flip,
            (false, false) => (), // allow flip or not,
        }
        if !force && flip {
            res *= 2;
        }
        if force && !flip {
            res /= 2;
        }
    }

    println!("{}", res);
}
