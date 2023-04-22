pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let _n = get_stdin_line().parse::<usize>().unwrap();
    let s = get_stdin_line();
    let mut seen_pipe = 0;
    for c in s.chars() {
        match c {
            '|' => seen_pipe += 1,
            '.' => (),
            '*' => {
                if seen_pipe == 1 {
                   println!("in");
                } else {
                   println!("out");
                }
                break;
            }
            _ => panic!("invalid char"),
        }
    }
}
