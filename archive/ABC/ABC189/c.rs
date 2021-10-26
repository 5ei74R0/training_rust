// https://atcoder.jp/contests/abc189/tasks/abc189_c
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let mut ans = -1;
    
    // for l in [0, n)
    for l in 0..n {
        // search best r from [l, n)
        let mut min = a[l];
        for r in l..n {
            min = std::cmp::min(min, a[r]);
            let sum = min * (r - l + 1) as i64;
            ans = std::cmp::max(ans, sum);
        }
    }

    assert_ne!(ans, -1);

    println!("{}", ans);
}
