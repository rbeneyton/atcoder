// use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let t = get_stdin_line().parse::<usize>().unwrap();
    for _ in 0..t {
        let mut x = get_stdin_line().split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        debug_assert_eq!(x.len(), 3);
        x.sort();

        if x[1] - x[0] % 2 == 1
        || x[2] - x[1] % 2 == 1
        {
            println!("-1");
            continue;
        }
        let d = x[2] - x[0];
        if d % 2 != 0
        {
            println!("-1");
            continue;
        }

        x[2] -= x[0];
        x[1] -= x[0];
        x[0] = 0;

        x[2] /= 2;
        x[1] /= 2;

        // let d = (x[2] - x[1]) + (x[2] - x[0]);
        // let mut turn = d / (2 + 4);
        // // extra swap required?
        // if turn != 0 && (x[0] == x[1] || x[2] == x[1]) {
        //     turn += 1;
        // }
        // println!("{}", turn);

        let mut turn = 0;
        loop {
            if x[0] == x[1] && x[1] == x[2] {
                break;
            }
            x.sort();
            x[0] += 3;
            x[1] += 2;
            x[2] += 1;
            turn += 1;
        };
        println!("{}", turn);

    }
}
