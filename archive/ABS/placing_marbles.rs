// https://atcoder.jp/contests/abs/tasks/abc081_a
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = 0;
    for ch in s {
        if ch == '1' {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
