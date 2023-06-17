use itertools::Itertools;

pub fn get_stdin_line() -> String {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();

    #[derive(Default, Copy, Clone, PartialEq)]
    pub struct Cand {
        sum: i64,
        upset: bool, // TODO upper bit of sum to gain 2 mem factor
    }
    impl Eq for Cand {}
    impl PartialOrd for Cand {
        fn partial_cmp(&self, other: &Cand) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Cand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self
                .sum.cmp(&other.sum)
                .then_with(|| other.upset.cmp(&self.upset)) // no upset is better
        }
    }
    let max_threshold = 500_usize;
    let mut nodes = Vec::with_capacity(max_threshold);
    nodes.push(Cand::default());
    let mut nodes_next = Vec::with_capacity(max_threshold);

    for _ in 0..n {
        let (x, y) = get_stdin_line().split(' ')
            .map(|x| x.to_string())
            .map(|x| x.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        let poison = x == 1;
        nodes_next.clear();
        for node in &nodes {
            let sum = node.sum + y;
            match (node.upset, poison) {
                (true, true) => (), // not die path
                (true, false) => nodes_next.push( Cand { sum, upset: false }), // recovered
                (false, true) => nodes_next.push( Cand { sum, upset: true }), // become ill
                (false, false) => nodes_next.push( Cand { sum, upset: false }), // stay fine
            }
            nodes_next.push(*node); // skip case
        }

        std::mem::swap(&mut nodes, &mut nodes_next);
        nodes.sort_by(|a, b| b.cmp(a));  // reverse
        nodes.dedup(); // drop duplicate
        nodes.truncate(max_threshold); // poor beam search /o\
    }
    println!("{}", nodes[0].sum);
}
