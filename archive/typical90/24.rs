// https://atcoder.jp/contests/typical90/tasks/typical90_x
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n]
    }
    
    let mut sum_diff = 0;
    for i in 0..n {
        sum_diff += (a[i] - b[i]).abs();
    }

    if sum_diff > k {
        println!("No");
    } else if (sum_diff - k) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }

}
