use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

pub fn substring(s: &String, l: usize, r: usize) -> String {
    s.chars().skip(l).take(r + 1 - l).collect()
}

#[test]
fn test_substring() {
    assert_eq!(substring(&String::from("ABC"), 0, 1), String::from("AB"));
    assert_eq!(substring(&String::from("ABC"), 0, 2), String::from("ABC"));
    assert_eq!(substring(&String::from("ABC"), 1, 2), String::from("BC"));
    assert_eq!(substring(&String::from("ABC"), 1, 1), String::from("B"));
}

pub fn perm(s: &String, l: usize, r: usize, x: char, y: char, z: char) -> String {
    s
        .chars()
        .enumerate()
        .map(|(idx, c)|
            if idx >= l && idx <= r {
                match c {
                    'A' => x,
                    'B' => y,
                    'C' => z,
                    _ => panic!(),
                }
            } else {
                c
            })
        .collect()
}

#[test]
fn test_perm() {
    assert_eq!(perm(&String::from("AAA"), 0, 2, 'A', 'B', 'C'), String::from("AAA"));
    assert_eq!(perm(&String::from("AAA"), 0, 2, 'B', 'A', 'C'), String::from("BBB"));
    assert_eq!(perm(&String::from("AAA"), 0, 0, 'B', 'A', 'C'), String::from("BAA"));
    assert_eq!(perm(&String::from("AAB"), 1, 2, 'C', 'A', 'B'), String::from("ACA"));
}

pub fn longest_chain(s: &String) -> (char, usize, usize) {
    let mut cur_c = ' ';
    let mut cur_len = 0;
    let mut cur_start = 0;
    let mut best_c = ' ';
    let mut best_len = 0;
    let mut best_start = 0;
    for (idx, c) in s.chars().enumerate() {
        if c == cur_c {
            cur_len += 1;
            if cur_len > best_len {
                best_len = cur_len;
                best_c = cur_c;
                best_start = cur_start;
            }
        } else {
            cur_c = c;
            cur_len = 1;
            cur_start = idx;
        }
    }
    (best_c, best_start, best_len)
}

#[test]
fn test_longest_chain() {
    assert_eq!(longest_chain(&String::from("AAA")), ('A', 0, 3));
    assert_eq!(longest_chain(&String::from("BBAAA")), ('A', 2, 3));
    assert_eq!(longest_chain(&String::from("AAABB")), ('A', 0, 3));
    assert_eq!(longest_chain(&String::from("AAABBAAAA")), ('A', 5, 4));
    assert_eq!(longest_chain(&String::from("AAABBAAAAB")), ('A', 5, 4));
}

pub type Simple = Vec::<(char, usize)>;

pub fn simple_chain(s: &String) -> Simple {
    let mut res = Vec::new();

    let mut cur_c = s.chars().next().unwrap();
    let mut cur_len = 1;
    for c in s.chars().skip(1) {
        if c == cur_c {
            cur_len += 1;
        } else {
            res.push((cur_c, cur_len));
            cur_c = c;
            cur_len = 1;
        }
    }
    res.push((cur_c, cur_len));
    res
}

#[test]
fn test_simple_chain() {
    assert_eq!(simple_chain(&String::from("AAA")), vec![('A', 3)]);
    assert_eq!(simple_chain(&String::from("BBAAA")), vec![('B', 2), ('A', 3)]);
    assert_eq!(simple_chain(&String::from("AAABB")), vec![('A', 3), ('B', 2)]);
    assert_eq!(simple_chain(&String::from("AAABBAAAA")), vec![('A', 3), ('B', 2), ('A', 4)]);
    assert_eq!(simple_chain(&String::from("AAABBAAAAB")), vec![('A', 3), ('B', 2), ('A', 4), ('B', 1)]);
}

pub fn simple_longest_chain(s: &Simple) -> usize {
    s
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.1.cmp(&b.1))
        .map(|(idx, _)| idx)
        .unwrap()
}

#[test]
fn test_simple_longest_chain() {
    assert_eq!(simple_longest_chain(&vec![('A', 3)]), 0);
    assert_eq!(simple_longest_chain(&vec![('B', 2), ('A', 3)]), 1);
    assert_eq!(simple_longest_chain(&vec![('A', 3), ('B', 2)]), 0);
    assert_eq!(simple_longest_chain(&vec![('A', 3), ('B', 2), ('A', 4)]), 2);
    assert_eq!(simple_longest_chain(&vec![('A', 3), ('B', 2), ('A', 4), ('B', 1)]), 2);
}

pub fn perm_on_simple(s: &Simple, t: usize, l: usize, x: char, y: char, z: char) -> Simple {
    s
        .iter()
        .enumerate()
        .map(|(idx, i)|
            if idx >= t && idx <= l {
                match i.0 {
                    'A' => (x, i.1),
                    'B' => (y, i.1),
                    'C' => (z, i.1),
                    _ => panic!(),
                }
            } else {
                (i.0, i.1)
            })
        .collect()
}

#[test]
fn test_perm_on_simple() {
    assert_eq!(perm_on_simple(&vec![('A', 1), ], 0, 0, 'A', 'B', 'C'),
                              vec![('A', 1), ]);
    assert_eq!(perm_on_simple(&vec![('A', 1), ], 0, 0, 'B', 'A', 'C'),
                              vec![('B', 1), ]);
    assert_eq!(perm_on_simple(&vec![('A', 9), ], 0, 0, 'B', 'A', 'C'),
                              vec![('B', 9), ]);
    assert_eq!(perm_on_simple(&vec![('A', 1), ('B', 1), ('C', 1), ], 0, 1, 'A', 'B', 'C'),
                              vec![('A', 1), ('B', 1), ('C', 1), ]);
    assert_eq!(perm_on_simple(&vec![('A', 1), ('B', 1), ('C', 1), ], 0, 1, 'B', 'A', 'C'),
                              vec![('B', 1), ('A', 1), ('C', 1), ]);
    assert_eq!(perm_on_simple(&vec![('A', 1), ('B', 1), ('C', 1), ], 1, 2, 'B', 'A', 'C'),
                              vec![('A', 1), ('A', 1), ('C', 1), ]);
    assert_eq!(perm_on_simple(&vec![('A', 1), ('B', 1), ('C', 1), ], 1, 2, 'B', 'C', 'A'),
                              vec![('A', 1), ('C', 1), ('A', 1), ]);
}

pub fn simple_factorize(s: &Simple) -> Simple {
    let mut res = Vec::new();
    res.reserve(s.len());

    let mut skip = false;
    for i in 0..(s.len()) {
        if skip {
            skip = false;
            continue;
        }
        if i != s.len() - 1 && s[i + 1].0 == s[i].0 {
            res.push((s[i].0, s[i].1 + s[i + 1].1));
            skip = true;
        } else {
            res.push((s[i].0, s[i].1));
        }
    }
    res
}

#[test]
fn test_simple_factorize() {
    assert_eq!(simple_factorize(&vec![('A', 1), ]),
                                 vec![('A', 1), ]);
    assert_eq!(simple_factorize(&vec![('A', 1), ('A', 1), ]),
                                 vec![('A', 2), ]);
    assert_eq!(simple_factorize(&vec![('A', 1), ('B', 1), ]),
                                 vec![('A', 1), ('B', 1), ]);
}

// pub fn beauty(s: &String) -> usize {
//     dbg!(s);
//     let mut w = simple_chain(s);
//     const PERMS : [(char, char, char); 6] = [('A', 'B', 'C'),
//                                              ('A', 'C', 'B'),
//                                              ('B', 'A', 'C'),
//                                              ('B', 'C', 'A'),
//                                              ('C', 'A', 'B'),
//                                              ('C', 'B', 'A')];
//     // 1 find longest stable chain
//     // 2 preserving this chain, change sides
//     let mut op = 0;
//
//     // left
//     loop {
//         let longest = simple_longest_chain(&w);
//         dbg!(longest);
//         if longest == 0 {
//             break;
//         }
//         let target = w[longest].0;
//         let perm = PERMS
//             .iter()
//             .filter(|(a, b, c)|
//                 (w[longest - 1].0 == 'A' && *a == target) ||
//                 (w[longest - 1].0 == 'B' && *b == target) ||
//                 (w[longest - 1].0 == 'C' && *c == target))
//             .filter(|(a, b, c)|
//                 // create join between -2 and -3 if possible
//                 if longest > 2 {
//                     (w[longest - 2].0 == 'A' && *a == w[longest - 3].0) ||
//                     (w[longest - 2].0 == 'B' && *b == w[longest - 3].0) ||
//                     (w[longest - 2].0 == 'C' && *c == w[longest - 3].0)
//                 } else {
//                     true
//                 })
//             .next()
//             .unwrap();
//         let left = if longest == 1 { 0 } else { longest - 2 };
//         let right = longest - 1;
//         let w_next = perm_on_simple(&w, left, right, perm.0, perm.1, perm.2);
//         dbg!(perm, left, right, &w_next);
//         op += 1;
//         w = simple_factorize(&w_next);
//     }
//     // right
//     loop {
//         let longest = simple_longest_chain(&w);
//         dbg!(longest);
//         if longest == w.len() - 1 {
//             break;
//         }
//         let target = w[longest].0;
//         let perm = PERMS
//             .iter()
//             .filter(|(a, b, c)|
//                 (w[longest + 1].0 == 'A' && *a == target) ||
//                 (w[longest + 1].0 == 'B' && *b == target) ||
//                 (w[longest + 1].0 == 'C' && *c == target))
//             .filter(|(a, b, c)|
//                 // create join between -2 and -3 if possible
//                 if longest + 3 < w.len() {
//                     (w[longest + 2].0 == 'A' && *a == w[longest + 3].0) ||
//                     (w[longest + 2].0 == 'B' && *b == w[longest + 3].0) ||
//                     (w[longest + 2].0 == 'C' && *c == w[longest + 3].0)
//                 } else {
//                     // opp side
//                     if longest + 3 < w.len() {
//                         (w[longest + 2].0 == 'A' && *a == w[longest + 3].0) ||
//                         (w[longest + 2].0 == 'B' && *b == w[longest + 3].0) ||
//                         (w[longest + 2].0 == 'C' && *c == w[longest + 3].0)
//                     } else {
//                         true
//                     }
//                 })
//             .next()
//             .unwrap();
//         let left = longest + 1;
//         let right = if longest == w.len() - 2 { longest + 1 } else { longest + 2 };
//         let w_next = perm_on_simple(&w, left, right, perm.0, perm.1, perm.2);
//         dbg!(perm, left, right, &w_next);
//         op += 1;
//         w = simple_factorize(&w_next);
//     }
//     dbg!(s, op);
//     op
// }

pub fn beauty(s: &String) -> usize {
    dbg!(s);
    let mut w = simple_chain(s);
    const PERMS : [(char, char, char); 6] = [('A', 'B', 'C'),
                                             ('A', 'C', 'B'),
                                             ('B', 'A', 'C'),
                                             ('B', 'C', 'A'),
                                             ('C', 'A', 'B'),
                                             ('C', 'B', 'A')];
    // 1 find longest stable chain
    // 2 preserving this chain, change sides
    let mut op = 0;

    loop {
        let longest = simple_longest_chain(&w);
        dbg!(longest);
        if longest == 0 && w.len() == 1 {
            break;
        }
        for left in (longest - 2)..(longest + 1) {
            for right in (longest - 1)..=(longest + 2) {
                let best_move = PERMS
                    .iter()
                    .map(|(a, b, c)|
                        let w_next = perm_on_simple(&w, left, right, perm.0, perm.1, perm.2);
                        dbg!(perm, left, right, &w_next);
                        op += 1;
                        w = simple_factorize(&w_next);
    }
    // right
    loop {
        let longest = simple_longest_chain(&w);
        dbg!(longest);
        if longest == w.len() - 1 {
            break;
        }
        let target = w[longest].0;
        let perm = PERMS
            .iter()
            .filter(|(a, b, c)|
                (w[longest + 1].0 == 'A' && *a == target) ||
                (w[longest + 1].0 == 'B' && *b == target) ||
                (w[longest + 1].0 == 'C' && *c == target))
            .filter(|(a, b, c)|
                // create join between -2 and -3 if possible
                if longest + 3 < w.len() {
                    (w[longest + 2].0 == 'A' && *a == w[longest + 3].0) ||
                    (w[longest + 2].0 == 'B' && *b == w[longest + 3].0) ||
                    (w[longest + 2].0 == 'C' && *c == w[longest + 3].0)
                } else {
                    // opp side
                    if longest + 3 < w.len() {
                        (w[longest + 2].0 == 'A' && *a == w[longest + 3].0) ||
                        (w[longest + 2].0 == 'B' && *b == w[longest + 3].0) ||
                        (w[longest + 2].0 == 'C' && *c == w[longest + 3].0)
                    } else {
                        true
                    }
                })
            .next()
            .unwrap();
        let left = longest + 1;
        let right = if longest == w.len() - 2 { longest + 1 } else { longest + 2 };
        let w_next = perm_on_simple(&w, left, right, perm.0, perm.1, perm.2);
        dbg!(perm, left, right, &w_next);
        op += 1;
        w = simple_factorize(&w_next);
    }
    dbg!(s, op);
    op
}

fn main() {
    let (n, q) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let s = get_stdin_line();
    dbg!(&s);
    debug_assert_eq!(s.len(), n);
    for _i in 1..=q {
        let (l, r) = get_stdin_line().split(' ')
            .map(|x| x.to_string())
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        // let ss = perm(&s, l - 1, r - 1, 'A', 'B', 'C');
        // let s = s[(l - 1)..r];
        let ss: String = substring(&s, l - 1, r - 1);
        // dbg!(l, r, &ss);
        let res = beauty(&ss);
        println!("{}", res);
    }
    // println!("{}", res);
}
