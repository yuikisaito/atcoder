use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut list = Vec::new();
    for ab in (0..s.len()).permutations(2) {
        let a = ab[0];
        let b = ab[1];
        list.push(s[a].clone() + &s[b]);
    }
    list.sort();
    list.dedup();
    println!("{}", list.len());
}
