// https://atcoder.jp/contests/typical90/tasks/typical90_v
use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64
    }

    // cube := (gcd of a, b, c)^3

    let p = gcd(gcd(a, b), c);
    let ans = (a / p - 1) + (b / p - 1) + (c / p - 1);

    println!("{}", ans);
}
