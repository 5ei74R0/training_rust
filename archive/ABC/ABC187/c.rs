// https://atcoder.jp/contests/abc187/tasks/abc187_c
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        vs: [Chars; n],
    }
    
    let mut ms = HashSet::new();

    for s in vs.iter().filter(|s| s[0] != '!') {
        ms.insert(&s[..]);
    }

    for exs in vs.iter().filter(|s| s[0] == '!') {
        let finding = &exs[1..];
        if ms.contains(finding) {
            let finding: String = finding.iter().collect();
            println!("{}", finding);
            return;
        }
    }

    println!("satisfiable");
}
