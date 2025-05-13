use itertools::{Itertools, iproduct};
use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    input! {
        r: usize,
        c: usize,
        x: usize,
        y: usize,
        d: usize,
        l: usize,
    }

    let xy_pattern = iproduct!(0..=r - x, 0..=c - y).count() % MOD;

    let dl_pattern =
        (0..x * y).combinations(d + l).count() * (0..d + l).combinations(d).count() % MOD;
    println!("{}", xy_pattern * dl_pattern % MOD)
}
