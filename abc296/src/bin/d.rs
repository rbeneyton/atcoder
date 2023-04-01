use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

// pub fn is_prime(n: u64) -> bool {
//   if n < 4 {
//     n > 1
//   } else if n % 2 == 0 || n % 3 == 0 {
//     false
//   } else {
//     let max_p = (n as f64).sqrt().ceil() as u64;
//     match (5..=max_p).step_by(6).find(|p| n % p == 0 || n % (p+2) == 0) {
//       Some(_) => false,
//       None => true
//     }
//   }
// }

fn main() {
    let (n, m) = get_stdin_line().split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect_tuple()
        .unwrap();

    if n == 1 || m == 1 {
        println!("1");
        return;
    }
    let mut res = std::u64::MAX;

    let mut b = 2;
    loop {
        if b > n {
            break;
        }
        let a = ((m as f64) / (b as f64)).ceil() as u64;
        if a <= n {
            let cand = a * b;
            // dbg!(format!("{}x{}={}", a, b, cand));
            if cand >= m && cand < res {
                res = cand;
                // fast exit
                if res == m { println!("{}", cand); return; }
            }
            b += 1;
        } else {
            b = std::cmp::max(
                b + 1,
                ((m as f64) / ((a - 1) as f64)).floor() as u64);
        }
    }
    if res != std::u64::MAX {
        println!("{}", res);
    } else {
        println!("-1");
    }
}
