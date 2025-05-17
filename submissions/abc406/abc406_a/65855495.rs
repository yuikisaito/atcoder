use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    println!(
        "{}",
        if c * 60 + d <= a * 60 + b {
            "Yes"
        } else {
            "No"
        }
    )
}
