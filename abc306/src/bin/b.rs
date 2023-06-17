// use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let res : u128 = get_stdin_line()
        .split(' ')
        .enumerate()
        .map(|(idx, x)|
            x.to_string()
                .parse::<u128>().unwrap()
            * 2u128.pow(idx as u32))
        .sum();
    println!("{}", res);
}
