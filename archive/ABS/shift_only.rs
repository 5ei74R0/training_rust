// https://atcoder.jp/contests/abs/tasks/abc081_b
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut min_div_times: i32 = 32;
    for val in a {
        let mut v = val;
        let mut cnt = 0;
        while v % 2 == 0 {
            v /= 2;
            cnt += 1;
        }
        min_div_times = std::cmp::min(min_div_times, cnt);
    }

    println!("{}", min_div_times);
}
