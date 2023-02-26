pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let mut x = get_stdin_line()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(x.len(), 5 * n);
    x.sort();
    let min = n;
    let max = 4 * n;
    let s : usize = x[min..max].iter().sum();
    let norm = 3 * n;
    let res = (s as f64) / (norm as f64);
    println!("{}", res);
}
