use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut crnt: Vec<usize> = vec![0, 0, 1];

    (4..=n).for_each(|_| {
        crnt.push(crnt.iter().rev().take(3).sum::<usize>() % 10007);
    });

    println!("{}", crnt[n - 1]);
}
