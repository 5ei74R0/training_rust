// https://atcoder.jp/contests/abc187/tasks/abc187_d
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    }
    ab.sort_by(|x, y| (y.0 * 2 + y.1).cmp(&(x.0 * 2 + x.1))); // inverse

    // maximize sum_b & minimize sum_a
    // -> maximize sum_b - sum_a

    let mut sum_a = 0;
    let mut sum_b = 0;

    for (a, _b) in &ab {
        sum_a += a;
    }

    for i in 0..n {
        sum_a -= ab[i].0;
        sum_b += ab[i].0 + ab[i].1;
        if sum_a < sum_b {
            println!("{}", i + 1);
            return;
        }
    }
}
