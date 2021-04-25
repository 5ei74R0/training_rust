// https://atcoder.jp/contests/abs/tasks/abc083_b
use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut ans = 0;
    for num in 1..n + 1 {
        let mut dsum = 0;
        let mut tmp = num;
        while tmp > 0 {
            dsum += tmp % 10;
            tmp /= 10;
        }
        if a <= dsum && dsum <= b {
            ans += num;
        }
    }
    println!("{}", ans);
}
