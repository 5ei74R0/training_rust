// https://atcoder.jp/contests/abc184/tasks/abc184_b
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        mut x: i32,
        s: Chars,
    }

    for c in s {
        x = match c {
            'o' => x + 1,
            'x' => std::cmp::max(0, x - 1),
            _ => x,
        }
    }
    println!("{}", x);
}
