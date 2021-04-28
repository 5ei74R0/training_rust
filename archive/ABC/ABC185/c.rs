// https://atcoder.jp/contests/abc185/tasks/abc185_c
use proconio::input;

fn main() {
    input! {
        l: i128,
    }
    let remain = l - 12;
    // ans = remain H 11 = Binomial(remain+11, 11)
    let mut ans = 1;
    for i in 0..11 {
        ans *= remain + 11 - i;
    }
    for i in 1..12 {
        ans /= i;
    }

    println!("{}", ans);
}
