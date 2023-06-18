// use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let t = get_stdin_line().parse::<usize>().unwrap();
    for _ in 0..t {
        let n = get_stdin_line().parse::<usize>().unwrap();
        let p = get_stdin_line().split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .map(|x| x - 1)
            .collect::<Vec<_>>();
        debug_assert_eq!(p.len(), n);
        debug_assert_eq!(*p.iter().min().unwrap(), 0);
        debug_assert_eq!(*p.iter().max().unwrap(), n - 1);
        eprintln!("p:{}", itertools::join(
            p.iter().map(|x| format!("{}", x)), ","));

        // // align on optimal starts
        // let mut arrivals = p.iter()
        //     .enumerate()
        //     .collect::<Vec<_>>();
        // arrivals.sort_by(|a, b| a.1.cmp(&b.1)); // sort by arrival order
        // // people i arrives the arrivals[i] ith
        // let arrivals = arrivals.iter()
        //     .map(|(idx, _)| *idx)
        //     .collect::<Vec<_>>();
        // debug_assert_eq!(arrivals.len(), n);
        // debug_assert_eq!(*arrivals.iter().min().unwrap(), 0);
        // debug_assert_eq!(*arrivals.iter().max().unwrap(), n - 1);
        // eprintln!("arrivals:{}", itertools::join(
        //     arrivals.iter().map(|x| format!("{}", x)), ","));

        // // optimal course durations
        // let mut offset = 0;
        // let mut offsets = Vec::with_capacity(n);
        // for i in 0..n {
        //     offset = std::cmp::max(offset, arrivals[i]);
        //     offset += 1;
        //     offsets.push(offset);
        // }
        // eprintln!("offsets:{}", itertools::join(
        //     offsets.iter().map(|x| format!("{}", x)), ","));

        // let min_return_time = offsets[p[0] as usize - 1];
        // eprintln!("first_return_time:{}", min_return_time);
        // for i in 1..n {
        //     let return_time = offsets[p[i] as usize - 1];
        //     eprintln!("first_return_time:{}", return_time);
        //     // if return_time != min_return_time {
        //     //     break;
        //     // }
        // }


        // // minimal return times when start times are consecutives
        // let min_return_time = p.iter()
        //     .enumerate()
        //     .map(|(idx, p)| p - (offsets[idx] as i32))
        //     .min()
        //     .unwrap();
        // eprintln!("min_return_time:{}", min_return_time);

        // // number of people with this potential time
        // let res = p.iter()
        //     .enumerate()
        //     .map(|(idx, p)| p - (offsets[idx] as i32))
        //     .filter(|x| *x == min_return_time)
        //     .count();
        // debug_assert!(res >= 1);
        // println!("{}", res);

        let mut start = (0..n).collect::<Vec<_>>();
        let mut res = 1;
        let mut min_duration = 0;
        for i in 1..n {
            // assume p[i] have same run time than p[i-1]
            let a = p[i - 1];
            let b = p[i];
            debug_assert!(a != b);
            if a < b {
                // we can to delay b (and remaining)
                let offset = start[b] - start[a];
                debug_assert!(offset > 0);
                for j in b..n { start[j] += offset; }
            } else {
                // we need to delay a (and remaining)
                let offset = start[a] - start[b];
                debug_assert!(offset > 0);
                for j in a..n { start[j] += offset; }
            }
            eprintln!("start:{}", itertools::join(start.iter().map(|x| format!("{}", x)), ","));
            // check
            let check = true;
            for still in 1..=i {
                eprintln!("offsets:{}", itertools::join(
                    (1..=i).map(|x| format!("{}", start[p[x]] as isize - start[p[0]] as isize)),
                    ","));
                // check |= ;
            }
        }
        println!("1");
    }
}
