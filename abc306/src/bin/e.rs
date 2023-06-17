use itertools::Itertools;
use std::collections::BTreeMap;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (n, k, q) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut a = Vec::new();
    a.resize(n, 0i64);

    let mut lower = BTreeMap::new();
    lower.insert(0i64, k);
    let mut lower_sum = 0i64;

    let mut upper = BTreeMap::new();
    upper.insert(0i64, n - k);

    let add = |map: &mut BTreeMap::<i64, usize>, v : i64| {
        map.entry(v).and_modify(|c| *c += 1).or_insert(1);
    };
    let remove = |map: &mut BTreeMap::<i64, usize>, v : &i64| -> bool {
        *map.get_mut(v).unwrap() -= 1;
        if *map.get(v).unwrap() == 0 {
            map.remove(v).expect("inconsistency");
            true
        } else {
            false
        }
    };

    for _ in 0..q {
        let (x, y) = get_stdin_line().split(' ')
            .map(|x| x.to_string())
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        debug_assert!(x > 0); // 1 based indices
        let x = x - 1;
        debug_assert!(x < n);
        let y = y as i64;

        let to_change = a[x];
        a[x] = y;

        // rev
        let to_change = -to_change;
        let y = -y;

        // remove 'to_change'
        debug_assert!(lower.values().cloned().sum::<usize>() == k);
        debug_assert!(upper.values().cloned().sum::<usize>() == n - k);
        debug_assert!(lower.contains_key(&to_change)
            || upper.contains_key(&to_change));
        let mut thr = *upper.keys().next().expect("inconsistency");
        if to_change >= thr {
            if remove(&mut upper, &to_change) {
                thr = *upper.keys().next().expect("inconsistency");
            }
        } else {
            remove(&mut lower, &to_change);
            lower_sum -= to_change;
            // flip one element from upper to lower to maintain count
            lower_sum += thr;
            add(&mut lower, thr);
            if remove(&mut upper, &thr) {
                thr = *upper.keys().next().expect("inconsistency");
            }
        }

        // add 'y'
        if y >= thr {
            add(&mut upper, y);
        } else {
            add(&mut lower, y);
            lower_sum += y;
            // flip one element from lower to upper to maintain count
            let thr_low = *lower.keys().rev().next().expect("inconsistency");
            assert!(thr_low <= thr);
            remove(&mut lower, &thr_low);
            lower_sum -= thr_low;
            add(&mut upper, thr_low);
        }
        println!("{}", -lower_sum);
    }
}
