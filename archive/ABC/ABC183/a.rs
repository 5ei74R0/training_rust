// https://atcoder.jp/contests/abc183/tasks/abc183_a
use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    println!("{}", if n > 0 { n } else { 0 });
}
