use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let msg = if a * b % 2 == 1 { "Odd" } else { "Even" };

    println!("{}", msg);
}
