use itertools::Itertools;
use rustc_hash::FxHashSet;
// use rustc_hash::FxHashMap;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (n, m) = get_stdin_line().split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    // let mut ids = FxHashMap::default();

    let mut vs = Vec::new();
    for _c in 0..m {
        let sn = get_stdin_line().parse::<usize>().unwrap();
        let mut s = FxHashSet::default();
        for v in get_stdin_line()
             .split(' ')
             .map(|x| x.parse::<usize>().unwrap())
        {
            s.insert(v - 1);

            // let followed = ids.entry(v).or_insert(FxHashSet::default());
            // followed.insert(c);
        }
        debug_assert_eq!(s.len(), sn);
        vs.push(s);
    }

    // if ids.len() != n {
    //     println!("0");
    //     return;
    // }
    let mut res = 0;
    let mut mask = Vec::new();
    for _ in 0..n {
        mask.push(false);
    }
    for sn in 1..(m+1) {
        for i in (0..m).combinations(sn) {
            for idx in 0..n {
                mask[idx] = false;
            }
            for c in i {
                for entry in &vs[c] {
                    mask[*entry] = true;
                }
            }
            if mask.iter().all(|x| *x) {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
