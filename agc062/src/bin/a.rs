use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

pub fn get_stdin_line_reused(tmp: &mut String) {
    tmp.clear();
    std::io::stdin().read_line(tmp).unwrap();
    tmp.truncate(tmp.trim_end().len());
}

// true => 'A', false => 'B'
#[derive(Debug, PartialEq, Eq)]
struct SString(Vec::<bool>);

impl SString {
    pub fn new() -> Self {
        Self { 0 : Vec::new() }
    }

    pub fn process_one(&mut self, tmp_as: &mut SString, tmp_bs: &mut SString) {
        let len = self.0.len();
        tmp_as.0.clear();
        tmp_bs.0.clear();
        tmp_as.0.reserve(len);
        tmp_bs.0.reserve(len);

        // let (mut as_next, mut bs_next) = (0, 0);
        for (c, c_next) in self.0.iter().tuple_windows() {
            match c {
                true => tmp_as.0.push(*c_next),
                false => tmp_bs.0.push(*c_next),
            }
            // if *c_next { as_next += 1; } else { bs_next += 1; }
            // // fast exit
            // if tmp_as.len() >= 2 && tmp_bs.len() >= 2 && as_next >= 2 && bs_next >= 2 {
            //     break;
            // }
        }

        self.0.clear();
        std::mem::swap(&mut self.0, &mut tmp_as.0);
        self.0.extend_from_slice(&tmp_bs.0);
        tmp_bs.0.clear();
    }

    pub fn process(&mut self, tmp_as: &mut SString, tmp_bs: &mut SString) {
        let len = self.0.len();
        for _i in 0..(len - 1) {
            self.process_one(tmp_as, tmp_bs);
        }
    }
}

impl From<&String> for SString {
    fn from(o: &String) -> Self {
        let mut res = Vec::with_capacity(o.len());
        for c in o.chars() {
            match c {
                'A' => res.push(true),
                'B' => res.push(false),
                _ => panic!("invalid char"),
            }
        }

        Self { 0 : res }
    }
}

impl From<&str> for SString {
    fn from(o: &str) -> Self {
        (&String::from(o)).into()
    }
}

impl Into<String> for SString {
    fn into(self) -> String {
        let mut res = String::with_capacity(self.0.len());
        for c in self.0.iter() {
            match c {
                true => res.push('A'),
                false => res.push('B'),
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_one() {
        let (mut tmp_as, mut tmp_bs) = (SString::new(), SString::new());

        let mut s : SString = "AB".into();
        s.process_one(&mut tmp_as, &mut tmp_bs);
        assert_eq!(s, "B".into());
        let mut s : SString = "ABAB".into();
        s.process_one(&mut tmp_as, &mut tmp_bs);
        assert_eq!(s, "BBA".into());
    }

    #[test]
    fn test_process() {
        let (mut tmp_as, mut tmp_bs) = (SString::new(), SString::new());

        let mut s : SString = "AB".into();
        s.process(&mut tmp_as, &mut tmp_bs);
        assert_eq!(s, "B".into());
        let mut s : SString = "ABAB".into();
        s.process(&mut tmp_as, &mut tmp_bs);
        assert_eq!(s, "A".into());
    }
}

fn main() {
    let t = get_stdin_line().parse::<usize>().unwrap();
    let mut tmp = String::with_capacity(3 * 10e5 as usize);
    let (mut tmp_as, mut tmp_bs) = (SString::new(), SString::new());
    for _i in 0..t {
        let _n = get_stdin_line().parse::<usize>().unwrap();
        get_stdin_line_reused(&mut tmp);
        let mut s : SString = (&tmp).into();
        s.process(&mut tmp_as, &mut tmp_bs);
        let s : String = s.into();
        println!("{}", s);
    }
}
