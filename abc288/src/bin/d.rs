use itertools::Itertools;
use ndarray::Array2;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

// {{{ direct solve with sliding buffer, works but TLE /o\

#[cfg(direct_computation)]
fn is_good_direct(a: &[i64], k: usize, tmp: &mut Vec::<i64>) -> bool
{
    let n = a.len();
    debug_assert!(n >= k);
    tmp.clear();
    tmp.resize(k, 0);

    let start = k * std::cmp::max(n.div_euclid(k) as isize - 2, 0) as usize;
    let start = 0;
    let stop = n - k + 1;
    let mut sum = 0; // rolling sum
    for i in start..n {
        sum -= tmp[i % k];
        if i < stop {
            let incr = -(a[i] + sum);
            tmp[i % k] = incr;
            sum += incr;
        } else {
            if a[i] + sum != 0 {
                return false;
            }
        }
    }
    true
}

// }}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(direct_computation)]
    fn test_is_good_direct() {
        let mut tmp = Vec::new();
        assert_eq!(is_good_direct(&[3,-1,1,-2,2,0], 3, &mut tmp), true);
        assert_eq!(is_good_direct(&[-1,1,-2,2,0,5], 3, &mut tmp), false);
    }
}

fn is_good(l: usize, r: usize, k: usize, sums: &Array2::<i64>) -> bool
{
    let sum_ref = sums[[0, r]] - sums[[0, l]];
    for ki in 1..k {
        if sum_ref != sums[[ki, r]] - sums[[ki, l]] {
            return false;
        }
    }
    true
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

    // sums[ki,ni] = sum of all 'k aligned' on left, shifted by ki
    let mut sums = Array2::zeros((k, n + 1));
    for ki in 0..k {
        for ni in 0..n {
            let off = ni as isize - ki as isize;
            let off = off % (k as isize);
            let incr = if off == 0 { a[ni] } else { 0 };
            sums[[ki, ni + 1]] = sums[[ki, ni]] + incr;
        }
    }

    for _ in 0..q {
        let (l, r) = get_stdin_line().split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let good = is_good(l - 1, r, k, &sums);
        println!("{}", if good { "Yes" } else { "No" });
    }
}
