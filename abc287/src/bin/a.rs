pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    debug_assert_eq!(n % 2, 1);

    let (mut yes, mut no) = (0, 0);
    let response_for = "For";
    let response_against = "Against";
    for _ in 0..n {
        let line = get_stdin_line();
        if line == response_for {
            yes += 1;
        } else {
            if line == response_against {
                no += 1;
            }
        }
    }
    if yes > no {
        println!("Yes");
    } else {
        println!("No");
    }
}
