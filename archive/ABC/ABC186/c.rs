// https://atcoder.jp/contests/abc186/tasks/abc186_c
use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut cnt = 0;
    for a in 1..n + 1 {
        let mut include7 = false;
        let mut a10 = a;
        while a10 > 0 {
            include7 |= a10 % 10 == 7;
            a10 /= 10;
        }
        let mut a8 = a;
        while a8 > 0 {
            include7 |= a8 % 8 == 7;
            a8 /= 8;
        }
        cnt += if include7 { 1 } else { 0 }
    }

    println!("{}", n - cnt);
}
