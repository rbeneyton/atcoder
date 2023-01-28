pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();

    let mut ss = Vec::new();
    ss.reserve(n);
    for _ in 0..n {
        ss.push(get_stdin_line());
    }


    let mut lcp = Vec::new();
    lcp.resize(n * n, 0);

    for idx in 0..n {

        for jdx in 0..n {
            if idx == jdx { continue; }
            let mut sj = ss[jdx].chars();
            let mut si = ss[idx].chars();

            let mut j_lcp = 0;
            for _ in 0.. {
                if let (Some(ci), Some(cj)) = (si.next(), sj.next()) {
                    if ci == cj {
                        j_lcp += 1;
                        continue;
                    }
                }
                break;
            }
            lcp[idx * n + jdx] = j_lcp;
        }
    }

    for idx in 0..n {
        let mut max_lcp = 0;
        for jdx in 0..n {
            if idx == jdx { continue; }
            let j_lcp = lcp[idx * n + jdx];
            max_lcp = std::cmp::max(max_lcp, j_lcp);
        }
        println!("{}", max_lcp);
    }
}
