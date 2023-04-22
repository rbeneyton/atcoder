use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn compact(s: &String) -> Vec::<(char, usize)>
{
    let mut ss = Vec::new();
    let mut same_c = 1;
    for (c1, c2) in s.chars().tuple_windows() {
        if c1 == c2 {
            same_c += 1
        } else {
            ss.push((c1, same_c));
            same_c = 1;
        }
    }
    ss.push((s.chars().last().unwrap(), same_c));
    ss
}

#[test]
fn test_compact() {
    debug_assert_eq!(compact(&String::from("-")),
                     vec![('-', 1)]);
    debug_assert_eq!(compact(&String::from("0")),
                     vec![('0', 1)]);
    debug_assert_eq!(compact(&String::from("--")),
                     vec![('-', 2)]);
    debug_assert_eq!(compact(&String::from("00")),
                     vec![('0', 2)]);
    debug_assert_eq!(compact(&String::from("00-")),
                     vec![('0', 2), ('-', 1)]);
    debug_assert_eq!(compact(&String::from("00--")),
                     vec![('0', 2), ('-', 2)]);
    debug_assert_eq!(compact(&String::from("00--0")),
                     vec![('0', 2), ('-', 2), ('0', 1)]);
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let s = get_stdin_line();
    debug_assert_eq!(s.len(), n);

    let have_tiret = s.chars().filter(|x| *x == '-').count() > 0;
    if !have_tiret {
        println!("-1");
        return;
    }
    let have_o = s.chars().filter(|x| *x == 'o').count() > 0;
    if !have_o {
        println!("-1");
        return;
    }

    let ss = compact(&s);
    if ss.len() == 1 {
        println!("-1");
        return;
    }

    let longest = ss.iter()
        .filter(|(c, _l)| *c == 'o')
        .map(|(_c, l)| l)
        .max()
        .unwrap();
    println!("{}", longest);
}
