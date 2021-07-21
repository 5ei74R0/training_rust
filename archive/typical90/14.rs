// https://atcoder.jp/contests/typical90/tasks/typical90_n
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n]
    }

    a.sort_by(|x, y| x.cmp(y));
    b.sort_by(|x, y| x.cmp(y));

    let mut sum = 0;

    for i in 0..n {
        sum += (a[i] - b[i]).abs();
    }

    println!("{}", sum);
}
