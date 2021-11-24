// https://atcoder.jp/contests/abc216/tasks/abc216_d
use proconio::input;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Ball {
    color: i32,
    frm: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BallPair {
    color: i32,
    frm1: usize,
    frm2: usize,
}

#[derive(Debug)]
struct Balls {
    que: VecDeque<BallPair>,
    pool: HashMap<i32, usize>, // color -> frm
}

impl Balls {
    fn add(&mut self, color: i32, frm: usize) {
        if self.pool.contains_key(&color) {
            let frm_othr = *self.pool.get(&color).unwrap();
            self.que.push_back(BallPair {
                color: color,
                frm1: frm,
                frm2: frm_othr,
            });
            self.pool.remove(&color);
        } else {
            self.pool.insert(color, frm);
        }
    }

    fn pop(&mut self) -> Option<BallPair> {
        self.que.pop_front()
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut top_balls = Balls {
        que: VecDeque::new(),
        pool: HashMap::new(),
    };
    let mut pipes = vec![VecDeque::new(); m];
    for i in 0..m {
        input! {k: usize}
        for _ in 0..k {
            input! {a: i32}
            pipes[i].push_front(a);
        }
    }

    for i in 0..m {
        let ball = pipes[i].pop_front();
        if ball != None {
            let color = ball.unwrap();
            top_balls.add(color, i);
        }
    }

    let mut rest = n;
    while !top_balls.que.is_empty() {
        let ball_pair = top_balls.pop();
        rest -= 1;
        let ball_pair = ball_pair.unwrap();
        let next1 = pipes[ball_pair.frm1].pop_front();
        if next1 != None {
            let next1 = next1.unwrap();
            top_balls.add(next1, ball_pair.frm1);
        }
        let next2 = pipes[ball_pair.frm2].pop_front();
        if next2 != None {
            let next2 = next2.unwrap();
            top_balls.add(next2, ball_pair.frm2);
        }
    }
    println!("{}", if rest > 0 {"No"} else {"Yes"});
}
