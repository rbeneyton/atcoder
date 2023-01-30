use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let t = get_stdin_line().parse::<usize>().unwrap();

    'test_cases: for _ in 0..t {
        let (n, k) = get_stdin_line().split(' ')
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        let s = get_stdin_line().chars().collect::<Vec<_>>();
        let s_len = s.len();
        debug_assert_eq!(s_len as isize, n);

        // {{{ utility lambdas

        let is_pal = |start, stop| {
            let start = start as usize;
            let stop = stop as usize;
            let step = (stop - start) / 2;
            let mut start = s.iter().skip(start);
            let mut stop = s.iter().rev().skip(n as usize - stop);
            for _ in 0..step {
                if let Some(l) = start.next() {
                    if let Some(r) = stop.next() {
                        if l != r {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        };
        let is_eq = |start1, start2, depth| {
            let mut a = s.iter().skip(start1 as usize);
            let mut b = s.iter().skip(start2 as usize);
            for _ in 0..depth {
                if let Some(l) = a.next() {
                    if let Some(r) = b.next() {
                        if l != r {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            true
        };

        // }}}

        // S'
        // check 2 sub palindromes
        let d = n - k % (2 * n); // wrap up size
        let d = d.abs();
        if !is_pal(0, d) {
            println!("No"); continue 'test_cases;
        }
        if !is_pal(n - d, n) {
            println!("No"); continue 'test_cases;
        }
        // check overlapping subranges
        if !is_eq(0, d, n - d) {
            println!("No"); continue 'test_cases;
        }
        println!("Yes");

        // // let pal_len = k + s_len;
        // // let pal_len_2 = pal_len / 2;
        // let idx_start = 0; // if pal_len_2 > k { pal_len_2 - k } else { 0 };
        // let idx_stop = s_len; // if pal_len_2 > s_len { s_len } else { pal_len_2 };

        // let mut potential_mirror = Vec::new();
        // 'start_pos: for idx in idx_start..idx_stop {
        //     // check symmetry inside S
        //     for idx_pal in 0.. {
        //         if idx + idx_pal >= s_len { break; }
        //         let right = s[idx + idx_pal];
        //         if idx < idx_pal + 1 { break; }
        //         let left = s[idx - 1 - idx_pal];
        //         if left != right {
        //             continue 'start_pos;
        //         }
        //     }
        //     potential_mirror.push(idx);
        // }
        // dbg!(idx_t, s_len, k, itertools::join(potential_mirror.iter(), ","));
        // for m1 in &potential_mirror {
        //     let m1 = *m1;
        //     'm2_scan: for m2 in &potential_mirror {
        //         let m2 = *m2;
        //         if m2 <= m1 { continue; }
        //         debug_assert_eq!(s[0], s[2 * m1]);
        //         for k_idx in 0..k {
        //             if m1 + m1 + k_idx >= s_len { continue 'm2_scan; }
        //             let idx_left = m1 + m1 + k_idx; // mirror via m1
        //             let m2_dist = s_len + k - m2;
        //             if m2 < m2_dist { continue 'm2_scan; }
        //             let idx_right = m2 - m2_dist; // mirror via m2
        //             dbg!(m1, m2, k, k_idx, idx_left, idx_right, s[idx_left], s[idx_right]);
        //             if s[idx_left] != s[idx_right] { continue 'm2_scan; }
        //         }
        //         println!("Yes");
        //         continue 'test_cases;
        //     }
        // }
        // println!("No");
    }
}
