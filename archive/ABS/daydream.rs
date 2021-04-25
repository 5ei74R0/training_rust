// https://atcoder.jp/contests/abs/tasks/arc065_a
use proconio::input;
use proconio::marker::Chars;

fn substr_there(idx: usize, s: &Vec<char>, sub: &Vec<char>) -> bool {
    let mut res = true;
    for i in 0..sub.len() {
        if idx + i >= s.len() {
            res = false;
            break;
        }
        if s[idx + i] != sub[i] {
            res = false;
            break;
        }
    }
    res
}

fn main() {
    input! {
        s: Chars,
    }
    let mut dp = vec![false; s.len() + 1];
    let subs = vec![
        "dream".chars().collect(),
        "dreamer".chars().collect(),
        "erase".chars().collect(),
        "eraser".chars().collect(),
    ];
    dp[0] = true;
    for i in 0..s.len() {
        if !dp[i] {
            continue;
        }
        for sub in subs.iter() {
            if substr_there(i, &s, sub) {
                dp[i + sub.len()] = true;
            }
        }
    }
    println!("{}", if dp[s.len()] { "YES" } else { "NO" });
}
