// https://atcoder.jp/contests/abc188/tasks/abc188_d
use proconio::input;

fn solve1() { // my 1st solution
    input! {
        n: usize,
        c: i64
    }

    let mut map = std::collections::BTreeMap::new();

    for _ in 0..n {
        input! {
            val_a: i64,
            val_b: i64,
            val_c: i64,
        }
        // register an segment as half open [val_a, val_b + 1)
        map.entry(val_a)
            .and_modify(|e| *e += val_c)
            .or_insert(val_c); // start
        map.entry(val_b + 1)
            .and_modify(|e| *e -= val_c)
            .or_insert(-val_c); // end
    }
    // dbg!(map.clone());

    let mut sum = 0;
    for (_, v) in map.iter_mut() {
        sum += *v;
        *v = sum;
    }
    assert_eq!(sum, 0);
    // dbg!(map.clone());
    for (_, v) in map.iter_mut() {
        if *v > c {
            *v = c;
        }
    }

    let mut min_cost = 0;
    let mut prev_p = 0; // < 1
    let mut prev_cost = 0;
    for (p, v) in map.iter() {
        min_cost += prev_cost * (*p - prev_p);
        prev_p = *p; // update
        prev_cost = *v; // update
    }

    println!("{}", min_cost);
}

fn solve2() { // my 2nd solution (40ms faster)
    input! {
        n: usize,
        c: i64
    }

    let mut vec = Vec::new();

    for _ in 0..n {
        input! {
            val_a: i64,
            val_b: i64,
            val_c: i64,
        }
        // register an segment as half open [val_a, val_b + 1)
        vec.push((val_a, val_c)); // start
        vec.push((val_b + 1, -val_c)); // end
    }
    vec.sort_by(|x, y| x.0.cmp(&y.0));
    // dbg!(vec.clone());

    let mut sum = 0;
    for (_, v) in vec.iter_mut() {
        sum += *v;
        *v = sum;
    }
    assert_eq!(sum, 0);
    // dbg!(vec.clone());
    for (_, v) in vec.iter_mut() {
        if *v > c {
            *v = c;
        }
    }

    let mut min_cost = 0;
    let mut prev_p = 0; // < 1
    let mut prev_cost = 0;
    for (p, v) in vec.iter() {
        min_cost += prev_cost * (*p - prev_p);
        prev_p = *p; // update
        prev_cost = *v; // update
    }

    println!("{}", min_cost);
}

fn main() {
    // solve1();
    solve2();
}
