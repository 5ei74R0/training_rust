// https://atcoder.jp/contests/abc185/tasks/abc185_b
use proconio::input;

#[derive(Debug)]
struct Buttery {
    current: i32,
    max: i32,
}

impl Buttery {
    fn discharge(&mut self, time: i32) -> bool {
        self.current -= time;
        if self.current <= 0 {
            false
        } else {
            true
        }
    }

    fn charge(&mut self, time: i32) {
        self.current = std::cmp::min(self.current + time, self.max);
    }
}

fn main() {
    input! {
        n: i32,
        m: i32,
        t: i32,
    }

    let mut buttery = Buttery { current: n, max: n };

    let mut now = 0;
    for _ in 0..m {
        input! {
            a: i32,
            b: i32,
        }
        if !buttery.discharge(a - now) {
            println!("No");
            return;
        }
        buttery.charge(b - a);
        now = b;
    }
    if buttery.discharge(t - now) {
        println!("Yes");
    } else {
        println!("No");
    }
    return;
}
