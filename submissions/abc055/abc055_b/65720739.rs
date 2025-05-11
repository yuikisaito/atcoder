use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut power = 1;
    for i in 1..=n {
        power *= i;
    }
    println!("{}", power % (10 ^ 9 + 7))
}
