// use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

pub fn is_prime(n: u64) -> bool {
  if n < 4 {
    n > 1
  } else if n % 2 == 0 || n % 3 == 0 {
    false
  } else {
    let max_p = (n as f64).sqrt().ceil() as u64;
    match (5..=max_p).step_by(6).find(|p| n % p == 0 || n % (p+2) == 0) {
      Some(_) => false,
      None => true
    }
  }
}

pub struct Prime {
  curr: u64,
  next: u64,
}

impl Prime {
  pub fn new() -> Prime {
    Prime {
      curr: 2,
      next: 3,
    }
  }
}

impl Iterator for Prime {
  type Item = u64;

  fn next(&mut self) -> Option<Self::Item> {
    let prime = self.curr;
    self.curr = self.next;
    loop {
      self.next += match self.next%6 {
        1 => 4,
        _ => 2,
      };
      if is_prime(self.next) {
        break;
      }
    }
    Some(prime)
  }
}

pub fn task(k: u64) -> u64 {
    // if is_prime(k) {
    //     return k;
    // }

    // let mut w = k;
    // for i in 2.. {
    //     dbg!(i);
    //     if w % i == 0 {
    //         w = w / i;
    //         dbg!(k, w, i);
    //         if w == 1 {
    //             dbg!("ok");
    //             return i;
    //         }
    //     }
    // }
    // unreachable!()

    use rustc_hash::FxHashMap;
    let mut decomp = FxHashMap::default();
    let mut w = k;
    for prime in Prime::new() {
        let mut f = 0;
        let factor = loop {
            if w % prime == 0 {
                w = w / prime;
                f += 1;
            } else {
                break f;
            }
        };
        if factor > 0 {
            decomp.insert(prime, factor);
            if w == 1 {
                break;
            }
        }
        // fast exit
        if is_prime(w) {
            decomp.insert(w, 1);
            break;
        }
    }
    debug_assert_eq!(k,
        decomp
            .iter()
            .map(|(k, v)| k.pow(*v))
            .fold(1, |acc, x| acc * x));

    dbg!(k);
    for n in 2.. {
        let mut rem = false;
        for (k, v) in decomp.iter_mut() {
            if *v > 0 {
                if n % k == 0 {
                    dbg!(n);
                    *v -= 1;
                    rem = *v > 0;
                    break;
                }
                rem = true;
            }
        }
        if !rem {
            return n;
        }
    }
    unreachable!();
    // decomp.iter()
    //     .map(|(k, v)| {
    //         // k.pow(*v) ensure result, but we can use all powers until v
    //         let mut acc = 0;
    //         for i in 1.. {
    //             acc += i;
    //             if acc >= *v {
    //                 return k.pow(i);
    //             }
    //         }
    //         unreachable!();
    //     })
    //     .max()
    //     .unwrap()

    // let mut set = HashSet::new();
    // let orig_k = k;
    // let mut k = k;
    // let mut max_prime_p = 1;
    // 'decomp: loop {
    //     for prime in Prime::new() {
    //         if set.contains(&prime) {
    //             continue;
    //         }
    //         dbg!((orig_k, k, prime));
    //         //let mut local_prime = 1;
    //         if k % prime == 0 {
    //             // local_prime *= prime;
    //             // max_prime_p = std::cmp::max(max_prime_p, local_prime);
    //             max_prime_p = std::cmp::max(max_prime_p, prime);
    //             k = k / prime;
    //             set.insert(prime);
    //
    //             if k == 1 {
    //                 break 'decomp;
    //             }
    //             break;
    //         }
    //     }
    // }
    // max_prime_p

    // unreachable!();
    // let mut res = 1;
    // loop {
    //     for n in 2.. {
    //         dbg!(n, k);
    //         if k % n == 0 {
    //             k = k / n;
    //             if k == 1 {
    //                 return std::cmp::max(n, res);
    //             }
    //             break;
    //         }
    //     }
    // }
}

fn main() {
    let k = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<u64>().unwrap())
        .next()
        .unwrap();

    println!("{}", task(k));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d() {
        assert_eq!(task(2), 2);
        assert_eq!(task(123456789011), 123456789011);
        assert_eq!(task(280), 7);
        assert_eq!(task(49), 49);
        assert_eq!(task(2*3*4), 4);
        assert_eq!(task(2*2*3*4), 8);
        assert_eq!(task(7), 7);
        assert_eq!(task(7*7), 7*7);
        assert_eq!(task(7*7*7), 7*7);
        assert_eq!(task(7u64.pow(5)), 7u64.pow(3));
        assert_eq!(task(7u64.pow(5)*3u64.pow(5)), 7u64.pow(3));
        assert_eq!(task(7u64.pow(5)*2u64.pow(10)), 7u64.pow(3));
        assert_eq!(task(7u64.pow(5)*5u64.pow(15)), 5u64.pow(5));
    }
}
