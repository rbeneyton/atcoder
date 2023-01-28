use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (mut h, mut m) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();


    let is_confusing = |h, m| {
        assert!(h < 24 && m < 60);
        let (a, b) = (h / 10, h % 10);
        let (c, d) = (m / 10, m % 10);

        let conf_h = 10 * a + c;
        let conf_m = 10 * b + d;

        conf_h < 24 && conf_m < 60
    };

    loop {
        if is_confusing(h, m) {
            println!("{} {}", h, m);
            break;
        }
        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
            if h == 24 {
                h = 0;
                // break;
            }
        }
    }
}
