pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let s = get_stdin_line()
        .chars()
        .map(|x| (x, x == '?'))
        .collect::<Vec<_>>();
    let t = get_stdin_line()
        .chars()
        .map(|x| (x, x == '?'))
        .collect::<Vec<_>>();

    let s_len = s.len();
    let t_len = t.len();
    debug_assert!(s_len > t_len);
    let hole_len = s_len - t_len;

    for x in 0..(t_len + 1) {
        let mut do_match = true;
        for idx in 0..t_len {
            let s_idx = if idx < x {
                idx
            } else {
                idx + hole_len
            };
            debug_assert!(s_idx < s_len);
            let s_prime_entry = s[s_idx];
            let t_entry = t[idx];

            match (s_prime_entry, t_entry) {
                ((sc, false), (tc, false)) => if sc != tc {
                    do_match = false;
                    break;
                },
                _ => (),
            }
        }
        if do_match {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
