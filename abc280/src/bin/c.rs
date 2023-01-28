// use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

pub fn task(s: String, t: String) -> usize {
    let mut si = s.chars();
    let mut ti = t.chars();
    let mut inserted = 1;
    loop {
        if let Some(sic) = si.next() {
            if let Some(tic) = ti.next() {
                if sic != tic {
                    break;
                }
                inserted += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    inserted
}

fn main() {
    let s = get_stdin_line();
    let t = get_stdin_line();

    println!("{}", task(s, t));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start() {
        assert_eq!(task(
            String::from("abc"),
            String::from("aabc")),
            2);
        assert_eq!(task(
            String::from("abc"),
            String::from("abca")),
            4);
        assert_eq!(task(
            String::from("abc"),
            String::from("xabc")),
            1);
    }
}
