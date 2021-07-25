// https://atcoder.jp/contests/typical90/tasks/typical90_t
use proconio::input;

fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128
    }

    let mut p = 1;
    for _ in 0..b {
        p *= c;
    }

    println!("{}", if a < p { "Yes" } else { "No" });
}
