use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    }

    let count = b / x - (a - 1) / x;
    println!("{}", count);
}
