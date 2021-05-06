// https://atcoder.jp/contests/abc186/tasks/abc186_d
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort_by(|x, y| x.cmp(y));

    let mut csum = vec![0; n + 1];
    for i in 0..n {
        csum[i + 1] = csum[i] + a[i];
    }

    let mut sum = 0;
    for i in 0..n - 1 {
        // // brute force
        // for j in i+1..n {
        //     sum += a[j] - a[i];
        // }

        // use cumulative sum
        sum += csum[n] - csum[i];
        sum -= a[i] * (n - i) as i64;
    }

    println!("{}", sum);
}
