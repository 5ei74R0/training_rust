// https://atcoder.jp/contests/abs/tasks/abc088_b
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort_by(|x, y| x.cmp(y).reverse());
    let mut alice = 0;
    let mut bob = 0;
    for (i, &val) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += val;
        } else {
            bob += val;
        }
    }
    println!("{}", alice - bob);
}
