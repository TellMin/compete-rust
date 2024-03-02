#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use superslice::Ext;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            n: usize,
            mut grid: [Chars; n],
        }

        let mut ans = grid.clone();

        for i in 0..n - 1 {
            ans[0][i + 1] = grid[0][i];
            ans[i + 1][n - 1] = grid[i][n - 1];
            ans[n - 1][n - 1 - i - 1] = grid[n - 1][n - 1 - i];
            ans[n - 1 - i - 1][0] = grid[n - 1 - i][0];
        }

        for line in ans {
            println!("{}", line.iter().join(""));
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
