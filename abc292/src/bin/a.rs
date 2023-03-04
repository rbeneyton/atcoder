pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let s = get_stdin_line();
    for c in s.chars() {
        print!("{}", c.to_uppercase());
    }
    println!("");
}
