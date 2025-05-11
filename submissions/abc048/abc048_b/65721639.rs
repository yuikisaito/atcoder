use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }

    let count = b / x - a / x + if a % x == 0 { 1 } else { 0 };
    println!("{}", count);
}
