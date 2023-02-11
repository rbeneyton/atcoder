use itertools::Itertools;
use std::iter;
use rustc_hash::FxHashSet;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let a = get_stdin_line()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(a.len(), n);
    let m = get_stdin_line().parse::<usize>().unwrap();
    let b = get_stdin_line()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<FxHashSet<_>>();
    debug_assert_eq!(b.len(), m);
    let x = get_stdin_line().parse::<usize>().unwrap();

    // all possible moves
    for v in (0..n)
        .map(|idx| 0..(x / a[idx]))
        .multi_cartesian_product()
    {
        let target : usize = v.iter()
            .enumerate()
            .map(|(idx, c)| a[idx] * c)
            .sum();
        // sum if correct
        if target == x {
            dbg!(itertools::join(
                v.iter().map(|x| format!("{}", x)),
                ","));
            // check if possible given all possibilities
            let steps = v.iter()
                .enumerate()
                .map(|(idx, c)| iter::repeat(a[idx]).take(*c))
                .flatten()
                .collect::<Vec<_>>();
            let n_steps = steps.len();
            'path: for path in steps
                .into_iter()
                .permutations(n_steps)
            {
                let mut pos = 0;
                for step in &path {
                    pos += step;
                    if b.contains(&pos) {
                        continue 'path;
                    }
                }
                dbg!(itertools::join(
                    path.iter().map(|x| format!("{}", x)),
                    ","));
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
