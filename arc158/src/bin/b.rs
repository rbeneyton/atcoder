use itertools::Itertools;
use std::cmp::Ordering::{Less, Greater};

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let x = get_stdin_line().split(' ')
        .map(|x| x.parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    debug_assert_eq!(x.len(), n);

    let mut min_value = None;
    let mut max_value = None;
    for (a, b, c) in x.iter().tuple_windows() {
        let v = (a+b+c) / (a*b*c);
        if min_value.is_none()
        || v.partial_cmp(&min_value.unwrap()).unwrap() == Less {
            min_value = Some(v);
        }
        if max_value.is_none()
        || v.partial_cmp(&max_value.unwrap()).unwrap() == Greater {
            max_value = Some(v);
        }
    }
    println!("{}", min_value.unwrap());
    println!("{}", max_value.unwrap());

    // x.sort_by(|(_, a), (_, b)| a.cmp(b));
    //
    // let mut xabs = x.iter()
    //     .map(|(idx,x)| (idx, x.abs()))
    //     .collect::<Vec<_>>();
    // xabs.sort_by(|(_, a), (_, b)| a.cmp(b));
    //
    // let mut min_denum = 1f32;
    // let mut min_iter = xabs.iter();
    // min_denum *= min_iter.next().unwrap().1 as f32;
    // min_denum *= min_iter.next().unwrap().1 as f32;
    // min_denum *= min_iter.next().unwrap().1 as f32;
    //
    // let mut max_denum = 1f32;
    // let mut max_iter = xabs.iter().rev();
    // max_denum *= max_iter.next().unwrap().1 as f32;
    // max_denum *= max_iter.next().unwrap().1 as f32;
    // max_denum *= max_iter.next().unwrap().1 as f32;
    //
    // let mut min_num = 0f32;
    // let mut min_iter = xabs.iter();
    // min_num += min_iter.next().unwrap().1 as f32;
    // min_num += min_iter.next().unwrap().1 as f32;
    // min_num += min_iter.next().unwrap().1 as f32;
    //
    // let mut max_num = 0f32;
    // let mut max_iter = xabs.iter().rev();
    // max_num += max_iter.next().unwrap().1 as f32;
    // max_num += max_iter.next().unwrap().1 as f32;
    // max_num += max_iter.next().unwrap().1 as f32;
    //
    // let min_value = min_num / max_denum;
    // println!("{}", min_value);
    //
    // let max_value = max_num / min_denum;
    // println!("{}", max_value);
}
