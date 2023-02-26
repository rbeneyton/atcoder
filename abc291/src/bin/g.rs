pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let a = get_stdin_line()
        .split(' ')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut b = get_stdin_line()
        .split(' ')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(a.len(), n);
    assert_eq!(b.len(), n);

    // wrap around b
    for idx in 0..n {
        b.push(b[idx]);
    }
    let b = b;


    // convert [u8] onto [u128] with 16 alignment cases
    // todo!();

    let mut best_sum = 0;
    for shift in 0..n {
        let mut sum = 0;
        for idx in 0..n {
            sum += a[idx] | b[idx + shift];
        }
        //dbg!(shift, sum);
        best_sum = std::cmp::max(best_sum, sum);
    }

    print!("{}", best_sum);
}
