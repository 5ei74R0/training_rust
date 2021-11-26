// https://atcoder.jp/contests/abc228/tasks/abc228_d
use proconio::input;

struct DSU {
    sizes: Vec<usize>,
    right_edge: Vec<usize>,
    parent: Vec<usize>,
    val: Vec<i64>,
}

impl DSU {
    fn new(size: usize) -> Self {
        let mut dsu = DSU {
            sizes: vec![0; size],
            val: vec![-1; size],
            right_edge: Vec::new(),
            parent: Vec::new(),
        };
        for i in 0..size {
            dsu.parent.push(i);
            dsu.right_edge.push(i);
        }
        dsu
    }

    fn same(&self, a: usize, b: usize) -> bool {
        self.parent[a] == self.parent[b]
    }

    fn merge(&mut self, l: usize, r: usize) {
        let root_r = self._get_root(r);
        let right_most = self.get_right_edge(root_r);

        let (larger, smaller) = {
            if self.sizeof(l) > self.sizeof(r) {
                (l, r)
            } else {
                (r, l)
            }
        };
        let root_s = self._get_root(smaller);
        let root = self._get_root(larger);
        self.parent[root_s] = root;
        self.sizes[root] += self.sizes[root_s];
        self.right_edge[root] = right_most;
    }

    fn sizeof(&mut self, idx: usize) -> usize {
        let root = self._get_root(idx);
        self.sizes[root]
    }

    fn get_right_edge(&mut self, idx: usize) -> usize {
        let root = self._get_root(idx);
        self.right_edge[root]
    }

    fn get_val(&self, idx: usize) -> i64 {
        self.val[idx]
    }

    fn set_val(&mut self, idx: usize, v: i64) {
        self.val[idx] = v;
    }

    fn _get_root(&mut self, mut idx: usize) -> usize {
        while idx != self.parent[idx] {
            self.parent[idx] = self.parent[self.parent[idx]];
            idx = self.parent[idx];
        }
        idx
    }
}

fn gen_n() -> usize {
    let mut n = 1;
    for _ in 0..20 {
        n *= 2;
    }
    n
}

fn main() {
    input! {
        q: usize,
    }
    let n = gen_n();

    let mut dsu = DSU::new(n);

    for _ in 0..q {
        input! {t: i32, mut x: usize}
        let h = x % n;
        match t {
            1 => {
                if dsu.get_val(h) == -1 {
                    dsu.set_val(h, x as i64);
                    let p_idx = (h-1) % n;
                    let n_idx = (h+1) % n;
                    if dsu.get_val(p_idx) != -1 {
                        dsu.merge(p_idx, h);
                    }
                    if dsu.get_val(n_idx) != -1 {
                        dsu.merge(h, n_idx);
                    }
                } else {
                    let right_edge = dsu.get_right_edge(h);
                    let n_idx = (right_edge + 1) % n;
                    dsu.set_val(n_idx, x as i64);
                    dsu.merge(h, n_idx);
                    let nn_idx = (n_idx + 1) % n;
                    if dsu.get_val(nn_idx) != -1 {
                        dsu.merge(h, nn_idx);
                    }
                }
            }
            2 => {
                println!("{}", dsu.get_val(h));
            }
            _ => panic![],
        }
    }
}
