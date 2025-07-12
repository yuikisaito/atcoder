use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
    }

    if m >= n {
        println!("0");
        return;
    }
    let min = x.iter().min().unwrap();
    let zero_start = x.iter().map(|xi| xi - min).sorted();
    let distances = zero_start
        .clone()
        .skip(1)
        .zip(zero_start.clone().take(zero_start.len() - 1))
        .map(|(a, b)| a - b);
    let divide = distances
        .clone()
        .enumerate()
        .sorted_by_key(|(_, di)| *di)
        .rev()
        .take(m - 1)
        .map(|(_, di)| di);

    println!("{}", distances.sum::<usize>() - divide.sum::<usize>());
}
