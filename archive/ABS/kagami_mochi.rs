// https://atcoder.jp/contests/abs/tasks/abc085_b
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [i32; n],
    }
    d.sort_by(|x, y| x.cmp(y));
    let mut prev = -1;
    let mut cnt = 0;
    for val in d {
        if val != prev {
            prev = val;
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
