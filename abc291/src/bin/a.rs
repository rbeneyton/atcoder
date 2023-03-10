pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let s = get_stdin_line();
    for (idx, c) in s.chars().enumerate() {
        if c.is_ascii_uppercase() {
            println!("{}", idx + 1);
        }
    }
    println!("");
}
