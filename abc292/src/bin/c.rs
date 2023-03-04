pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();

    let mut v = Vec::with_capacity(n);
    v.resize(n, 0usize);

    for a in 1..n {
        for b in 1..n {
            let s = a * b;
            if s >= n { break; }
            v[s] += 1;
        }
    }
    let mut res = 0;
    for s1 in 1..n {
        if v[s1] > 0 {
            let s2 = n - s1;
            if v[s2] > 0 {
                res += v[s1] * v[s2];
            }
        }
    }
    println!("{}", res);
}
