// https://atcoder.jp/contests/abc185/submissions/22757897
use proconio::input;

fn main() {
    let mut p = 1e10 as i64;
    for _ in 0..4 {
        input! {
            a: i64,
        }
        p = std::cmp::min(p, a);
    }

    println!("{}", p);
}
