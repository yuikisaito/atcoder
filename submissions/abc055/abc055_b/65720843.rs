use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut power = 1;
    for i in 1..=n {
        power *= i;
        power %= 1000000007;
    }
    println!("{}", power)
}
