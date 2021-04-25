// https://atcoder.jp/contests/abs/tasks/arc089_a
use proconio::input;

fn main() {
    input! {
        n: usize,
        plan: [[i32; 3]; n],
    }
    let mut t = 0;
    let mut x = 0;
    let mut y = 0;
    for p in plan.iter() {
        let tim = p[0] - t;
        let mv = (p[1] - x).abs() + (p[2] - y).abs();
        if tim < mv || tim % 2 != mv % 2 {
            println!("No");
            return;
        }
        t = p[0];
        x = p[1];
        y = p[2];
    }
    println!("Yes");
}
