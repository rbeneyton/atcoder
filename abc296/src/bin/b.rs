pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    for row in 0..=7 {
        let s = get_stdin_line();
        debug_assert_eq!(s.len(), 8);
        let mut si = s.chars();
        for col in 0..=7 {
            let c = si.next().unwrap();
            if c == '*' {
                let col = (('a' as u8) + col) as char;
                let row = 8 - row;
                println!("{}{}", col, row);
                return;
            }
        }
    }
    panic!();
}
