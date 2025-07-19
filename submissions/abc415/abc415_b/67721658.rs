use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let nimotsu = s
        .iter()
        .enumerate()
        .filter(|&(_, &si)| si == '#')
        .map(|(i, _)| i + 1)
        .collect_vec();
    for n in (0..nimotsu.len()).step_by(2) {
        println!("{},{}", nimotsu[n], nimotsu[n + 1]);
    }
}
