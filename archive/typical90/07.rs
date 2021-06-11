// https://atcoder.jp/contests/typical90/tasks/typical90_g
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
        b: [i32; q],
    }
    a.sort_by(|x, y| x.cmp(y));

    for i in 0..q {
        let b_val = b[i];
        let mut lb = -1;
        let mut ub = n as i32;
        while ub - lb > 1 {
            let mid = (lb + (ub - lb) / 2) as usize;
            if b_val <= a[mid] {
                ub = mid as i32;
            } else {
                lb = mid as i32;
            }
        }
        lb = std::cmp::max(lb, 0);
        ub = std::cmp::min(ub, n as i32 - 1);
        println!(
            "{}",
            std::cmp::min(
                (b_val - a[lb as usize]).abs(),
                (b_val - a[ub as usize]).abs()
            )
        );
    }
}
