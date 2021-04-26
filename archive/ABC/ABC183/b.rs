// https://atcoder.jp/contests/abc183/tasks/abc183_b
use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }
    println!("{}", sx + (gx - sx) / (gy + sy) * sy);
}
