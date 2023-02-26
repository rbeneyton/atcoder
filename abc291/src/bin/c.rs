use rustc_hash::FxHashSet;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let s = get_stdin_line();
    debug_assert_eq!(s.len(), n);

    let (mut x, mut y) = (0, 0);
    let mut positions = FxHashSet::default();
    positions.insert((x, y));

    for c in s.chars() {
        match c {
            'R' => x = x + 1,
            'L' => x = x - 1,
            'U' => y = y + 1,
            'D' => y = y - 1,
            _ => panic!(),
        }
        if positions.contains(&(x, y)) {
            println!("Yes");
            return;
        }
        positions.insert((x, y));
    }
    println!("No");
}
