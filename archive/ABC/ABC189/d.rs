// https://atcoder.jp/contests/abc189/tasks/abc189_d
// take care about overflow!
use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![vec![0_i64; 2]; n + 1]; // dp[i][0] := num of y_i, where y_i == False
    dp[0][0] = 1;
    dp[0][1] = 1;

    for i in 0..n {
        input! {s: Bytes}
        if s == "AND".as_bytes() {
            // when x_(i+1) is False
            dp[i + 1][0] += dp[i][0] + dp[i][1];
            // when x_(i+1) is True
            dp[i + 1][1] += dp[i][1];
            dp[i + 1][0] += dp[i][0];
        } else {
            // when x_(i+1) is False
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][1] += dp[i][1];
            // when x_(i+1) is True
            dp[i + 1][1] += dp[i][0] + dp[i][1];
        }
    }

    println!("{}", dp[n][1]);
}
