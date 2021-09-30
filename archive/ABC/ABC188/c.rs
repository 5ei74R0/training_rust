// https://atcoder.jp/contests/abc188/tasks/abc188_c
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }

    let mut siz = 1;
    for _ in 0..n {
        siz *= 2;
    }

    let mut que = VecDeque::new();
    for i in 0..siz {
        input! {a: i32}
        let pr = (a, i + 1);
        que.push_back(pr);
    }
    while siz > 2 {
        siz /= 2;
        for _ in 0..siz {
            let val1 = que.pop_front().unwrap();
            let val2 = que.pop_front().unwrap();
            que.push_back(std::cmp::max(val1, val2));
        }
    }

    let val1 = que.pop_front().unwrap();
    let val2 = que.pop_front().unwrap();
    let ans = std::cmp::min(val1, val2).1;

    println!("{}", ans);
}
