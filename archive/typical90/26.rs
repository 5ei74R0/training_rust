// https://atcoder.jp/contests/typical90/tasks/typical90_z
use proconio::input;

fn dfs(tree: &Vec<Vec<usize>>, color: &mut Vec<i32>, v: usize) {
    for i in 0..tree[v].len() {
        let next_v = tree[v][i];
        if color[next_v] != 0 {
            continue;
        }
        color[next_v] = if color[v] == 1 { 2 } else { 1 };
        dfs(tree, color, next_v);
    }
}

fn main() {
    // the tree is a bipartite graph
    // divide the vertices into 2 groups
    // choose N/2 vertices from the larger group

    input! {
        n: usize,
    }

    let mut tree = vec![vec![0; 0]; n];
    let mut color = vec![0; n];
    for _ in 0..n - 1 {
        input! { mut a: usize, mut b: usize }
        a -= 1;
        b -= 1;
        tree[a].push(b);
        tree[b].push(a);
    }
    color[0] = 1;
    
    dfs(&tree, &mut color, 0);

    let mut cnt_1 = 0;
    for i in 0..n {
        if color[i] == 1 {
            cnt_1 += 1;
        }
    }

    let majority = if cnt_1 >= n / 2 { 1 } else { 2 };

    let mut num = 0;
    for i in 0..n {
        if num >= n / 2 {
            break;
        }
        if color[i] == majority {
            num += 1;
            print!("{}{}", i + 1, if num < n / 2 { ' ' } else { '\n' });
        }
    }
}
