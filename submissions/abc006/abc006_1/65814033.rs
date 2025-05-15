use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        if n.to_string().contains("3") || n % 3 == 0 {
            "YES"
        } else {
            "NO"
        }
    )
}
