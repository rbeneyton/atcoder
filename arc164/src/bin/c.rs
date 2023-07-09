// {{{ usual stuff

#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

use std::{
    collections::*,
    fmt,
    io::{self, Write},
    mem::swap,
};
use itertools::Itertools;
use proconio::input;
use proconio::marker::{Bytes, Chars};

#[allow(unused_variables)]
const LOG_LVL : u8 = 0;

macro_rules! logstart {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("L{}:L{}:{}",
                line!(),
                lvl,
                fmt::format(format_args!($($arg)+)));
        }
    })
}
macro_rules! logcont {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprint!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
macro_rules! logstop {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprintln!("{}",
                fmt::format(format_args!($($arg)+)));
        }
    })
}
macro_rules! logln {
    ($lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= LOG_LVL {
            eprintln!("L{}:L{}:{}",
                line!(),
                lvl,
                fmt::format(format_args!($($arg)+)));
            io::stderr().flush().unwrap();
        }
    })
}

pub fn get_stdin_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_string()
}

// }}}

#[derive(Default, Clone)]
pub struct Node {
    // choices_alice: Vec<usize>,
    // choices_bob: Vec<usize>,
    va: Vec<u32>,
    vb: Vec<u32>,
    score_alice: u32,
    score_bob: u32,
    children: Nodes,
}
pub type Nodes = Vec<Node>;
impl Node {
    pub fn new(va: Vec<u32>, vb: Vec<u32>) -> Self {
        Self {
            // choices_alice: Vec::new(),
            // choices_bob: Vec::new(),
            va,
            vb,
            score_alice: 0,
            score_bob: 0,
            children: Vec::new(),
        }
    }
    pub fn play(&mut self) -> u32 {
        let mut score = 0;
        self.play_alice();
        for child_alice in &mut self.children {
            child_alice.play_bob();
            for child_bob in &mut child_alice.children {
                score = std::cmp::max(score, child_bob.play());
                score = std::cmp::max(score, child_bob.score_bob);
            }
            score = std::cmp::max(score, child_alice.score_bob);
        }
        score
    }
    pub fn play_alice(&mut self) {
        for idx in 0..(self.va.len()) {
            // let mut choices_alice_new = self.choices_alice.clone();
            // choices_alice_new.push(idx);

            let mut va_new = self.va.clone();
            let mut vb_new = self.vb.clone();
            std::mem::swap(&mut va_new[idx], &mut vb_new[idx]);

            self.children.push( Self {
                // choices_alice : choices_alice_new,
                // choices_bob : self.choices_bob.clone(),
                va : va_new,
                vb : vb_new,
                score_alice : self.score_alice,
                score_bob : self.score_bob,
                children: Vec::new(),
            });
        }
    }
    pub fn play_bob(&mut self) {
        for idx in 0..(self.va.len()) {
            let mut va_new = self.va.clone();
            let mut vb_new = self.vb.clone();
            let score = self.score_bob + va_new[idx];
            // logln!(0, "score bob: {}→{}", self.score_bob, score);
            va_new.swap_remove(idx);
            vb_new.swap_remove(idx);

            // let mut choices_bob_new = self.choices_bob.clone();
            // choices_bob_new.push(idx);
            self.children.push( Self {
                // choices_alice : self.choices_alice.clone(),
                // choices_bob : choices_bob_new,
                va : va_new,
                vb : vb_new,
                score_alice : self.score_alice,
                score_bob : score,
                children: Vec::new(),
            });
        }
    }
}

fn solve_c(_n : usize, va : Vec<u32>, vb : Vec<u32>) -> u32 {
    let mut root = Node::new(va, vb);

    root.play()
}

fn old_solve_c(_n : usize, mut va : Vec<u32>, mut vb : Vec<u32>) -> u32 {
    let mut score = 0;
    loop {
        // First, Alice chooses one of the remaining cards and flips it over.

        // Alice tries to minimize Bob's score at the end of the game
        // ⇒ chose greater card
        let (alice_idx, alice_v) = va.iter()
            .enumerate()
            .filter(|(idx, _)| vb[*idx] >= va[*idx]) // do not increase value
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .or(Some((0, &va[0])))
            .unwrap();
        logln!(0, "alice pick idx:{} with v:{}, so {}→{}",
            alice_idx, alice_v,
            va[alice_idx], vb[alice_idx]);
        std::mem::swap(&mut va[alice_idx], &mut vb[alice_idx]);

        // Next, Bob removes one of the remaining cards.
        // Then, Bob scores points equal to the number written on the face-up side of the removed card.

        // optimal: bob peek higher card
        let (bob_idx, bob_v) = va.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(&b))
            .unwrap();
        score += bob_v;
        logln!(0, "bob pick idx:{} with v:{}, score:{}",
            bob_idx, bob_v, score);

        va.swap_remove(bob_idx);
        vb.swap_remove(bob_idx);
        logln!(0, "va: {}", itertools::join(va.iter().map(|x| format!("{}", x)), ","));
        logln!(0, "vb: {}", itertools::join(va.iter().map(|x| format!("{}", x)), ","));

        if va.len() == 0 {
            return score;
        }
    }
}

#[test]
fn test_solve_c()
{
    assert_eq!(solve_c(3, vec![6, 2, 5], vec![4, 1, 3]), 12);

    assert_eq!(solve_c(5,
        vec![
            166971716,
            219878198,
            918378176,
            610749017,
            701849287,
        ],
        vec![
            552987438,
            619875818,
            518975015,
            285601372,
            307601390,
        ]),
        3078692091);
}

// #[proconio::fastout]
fn main() {
    let n = get_stdin_line().parse::<usize>().unwrap();
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for _ in 0..n {
        let (ai, bi) = get_stdin_line().split(' ')
            .map(|x| x.to_string())
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        a.push(ai);
        b.push(bi);
    }
    println!("{}", solve_c(n, a, b));
}

