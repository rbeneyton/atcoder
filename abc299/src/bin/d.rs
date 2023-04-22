use std::io::Write;
use std::io::stdout;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();

    let mut s = [255u8; 26];
    s[0] = 0;
    if n < s.len() {
        s[n - 1] = 1;
    }
    if n == 2 {
        println!("! 1");
        stdout().flush().unwrap();
        return;
    }
    for i in 1..21 {
        println!("? {}", i + 1);
        stdout().flush().unwrap();

        use std::{thread, time};
        let ten_millis = time::Duration::from_millis(20);
        thread::sleep(ten_millis);

        let response = get_stdin_line().parse::<u8>().unwrap();
        assert!(response <= 1);
        s[i] = response;
        if s[i - 1] != response {
            println!("! {}", i);
            stdout().flush().unwrap();
            return;
        }
        if s[i + 1] <= 1 && s[i + 1] != response {
            println!("! {}", i + 1);
            stdout().flush().unwrap();
            return;
        }
    }
}
