// https://atcoder.jp/contests/typical90/tasks/typical90_p
//
// Note!
// there is more optical algorithm
// we can use extended gcd of b & c to recognize whether there are feasible pair for chosen a.
// then we do not have to do more than one for-loop.
use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut ans = 10000;
    for i in 0..10000 as i64 {
        for j in 0..(10000 - i) as i64 {
            let sum = a * i + b * j;
            if sum > n {
                break;
            }
            if (n - sum) % c == 0 {
                ans = std::cmp::min(ans, i + j + (n - sum) / c);
            }
        }
    }

    println!("{}", ans);
}
