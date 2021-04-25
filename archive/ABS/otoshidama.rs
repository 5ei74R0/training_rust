// https://atcoder.jp/contests/abs/tasks/abc085_c
use proconio::input;

fn main() {
    input! {
        n: i32,
        mut y: i32,
    }
    let y = y / 1000;
    for i in 0..n + 1 {
        for j in 0..n + 1 - i {
            let k = y - 10 * i - 5 * j;
            if i + j + k == n {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }
    println!("-1 -1 -1");
    return;
}
