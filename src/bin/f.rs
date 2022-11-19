pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let a = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.chars())
        .map(|x| (x[0], x[x.len() - 1]))
        .map(|x| Some(x))
        .collect::<Vec<_>>();

    let optimal = |a| {
        let mut best = None;
        'main for i in 0..n {
            if let Some((_, ci)) = a[i] {
                for j in 0..n {
                    if j == i { continue; }
                    if let Some((cj, _)) = a[i] {
                        if ci == cj {
                            continue 'main;
                        }
                    }
                }
            }
            // no next move possible
            best = Some(i);
            break;
        }
    }
}
