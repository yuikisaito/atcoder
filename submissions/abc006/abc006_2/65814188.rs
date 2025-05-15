use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut crnt: Vec<usize> = vec![0, 0, 1];

    if n <= 2 {
        println!("{}", crnt[n] % 10007);
        return;
    }

    for _ in 3..=n {
        crnt.remove(0);
        crnt.push((crnt[0] + crnt[1] + crnt[2]) % 10007);
    }

    println!("{}", crnt[2]);
}
