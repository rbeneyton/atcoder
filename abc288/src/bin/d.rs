use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn is_good(a: &[i64], k: usize) -> bool
{
    let n = a.len();
    debug_assert!(n >= k);
    let mut a = a.to_vec();
    dbg!(&a, k);

    let start = if n + 1 >= 2 * k { n - 2 * k + 1 } else { 0 };
    let stop = n - k + 1;
    for i in start..stop {
        let incr = -a[i];
        for w in 0..k { a[i + w] += incr; }
        debug_assert_eq!(a[i], 0);
    }
    dbg!(&a);
    for i in a[start..].iter() {
        if *i != 0 {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_good() {
        assert_eq!(is_good(&[3,-1,1,-2,2,0], 3), true);
        assert_eq!(is_good(&[-1,1,-2,2,0,5], 3), false);
        assert_eq!(is_good(&[-1,1,-2,2,0,5], 3), false);
    }
}
fn main() {
    let (n, k) = get_stdin_line().split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let a = get_stdin_line().split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(a.len(), n);
    let q = get_stdin_line().parse::<usize>().unwrap();
    for _ in 0..q {
        let (l, r) = get_stdin_line().split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let good = is_good(&a[(l - 1)..r], k);
        println!("{}", if good { "Yes" } else { "No" });
    }
}
