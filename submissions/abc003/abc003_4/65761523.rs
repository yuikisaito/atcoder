use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        x: usize,
        y: usize,
        d: usize,
        l: usize,
    }

    let xy_pattern = (r - x + 1) * (c - y + 1);

    let dl_pattern = (0..x * y).combinations(d + l).count() * (0..d + l).combinations(d).count();

    println!("{}", xy_pattern * dl_pattern % 1000000007)
}
