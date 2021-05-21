// https://atcoder.jp/contests/arc115/tasks/arc115_b
use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [[i32; n]; n],
    }

    let mut a = vec![0; n];
    let mut b = vec![0; n];

    let mut min_c = 2e9 as i32;
    let mut min_ij = (0, 0);
    for (i, v) in c.iter().enumerate() {
        for (j, val) in v.iter().enumerate() {
            if min_c > *val {
                min_c = *val;
                min_ij = (i, j);
            }
        }
    }
    a[min_ij.0] = 0;
    b[min_ij.1] = min_c;

    for i in 0..n {
        a[i] = c[i][min_ij.1] - b[min_ij.1];
    }

    for j in 0..n {
        b[j] = c[min_ij.0][j] - a[min_ij.0];
    }

    for i in 0..n {
        for j in 0..n {
            if c[i][j] != a[i] + b[j] {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");

    for i in 0..n {
        print!("{}{}", a[i], if i == n - 1 { '\n' } else { ' ' });
    }

    for i in 0..n {
        print!("{}{}", b[i], if i == n - 1 { '\n' } else { ' ' });
    }
}
