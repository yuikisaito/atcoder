use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        c: [[char; 4]; 4],
    }

    c.iter()
        .rev()
        .map(|row| row.iter().rev())
        .for_each(|row| println!("{}", row.clone().join(" ")));
}
