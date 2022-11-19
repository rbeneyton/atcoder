pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let mut a = get_stdin_line().split(' ')
        .map(|x| x.to_string())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(a.len(), n);
    let q = get_stdin_line().parse::<usize>().unwrap();

    let mut def = None;
    for _i in 0..q {
        let inputs = get_stdin_line().split(' ')
            .map(|x| x.to_string())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        if inputs[0] == 1 {
            // memset
            let v = inputs[1] as u64;
            def = Some(v);
        }
        if inputs[0] == 2 {
            // incr
            let ai = inputs[1];
            assert!(ai >= 1 && ai <= n);
            let incr = inputs[2] as u64;
            // lazy fill
            if let Some(def) = def {
                for i in 0..n {
                    a[i] = def;
                }
            }
            a[ai - 1] += incr;
            def = None;
        }
        if inputs[0] == 3 {
            // print
            let ai = inputs[1];
            assert!(ai >= 1 && ai <= n);
            if let Some(def) = def {
                println!("{}", def);
            } else {
                println!("{}", a[ai - 1]);
            }
        }
    }
}
