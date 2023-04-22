use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let (_n, t) = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let t = t as u32;
    let c = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let r = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(c.len(), r.len());

    let color_t_played = c.iter().filter(|x| **x == t).count() > 0;
    let color_to_filter = if color_t_played { t } else { c[0] };
    let (win_idx, _) = r.iter().zip(c.iter())
        .enumerate()
        .filter(|(_idx, (_r, c))| **c == color_to_filter)
        .map(|(idx, (r, _c))| (idx, r))
        .fold((0, 0), |(max_idx, max_v), (idx, v)|
            if *v > max_v {
                (idx, *v)
            } else {
                (max_idx, max_v)
            });
    println!("{}", win_idx + 1);
}
