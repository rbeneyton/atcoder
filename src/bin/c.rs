use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (_n, q) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut follow = HashMap::new();

    for _i in 0..q {
        let (t, a, b) = get_stdin_line().split(' ')
            .map(|x| x.to_string())
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let followed = follow.entry(a).or_insert(HashSet::new());
        if t == 1 {
            followed.insert(b);
        }
        if t == 2 {
            followed.remove(&b);
        }
        if t == 3 {
            let res_1 = followed.contains(&b);
            drop(followed);
            if res_1 {
                if let Some(rev_followed) = follow.get(&b) {
                    if rev_followed.contains(&a) {
                        println!("Yes");
                        continue;
                    }
                }
            }
            println!("No");
        }
    }
}
