use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut crnt: Vec<usize> = vec![0, 0, 1];

    if n <= 3 {
        println!("{}", crnt[n] % 10007);
        return;
    }

    for _ in 4..=n {
        crnt.push(crnt.iter().sum::<usize>());
        crnt.remove(0);
    }

    println!("{}", crnt[2] % 10007);
}
