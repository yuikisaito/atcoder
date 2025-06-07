use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        t: Chars,
        a: Chars,
    }

    let result = t.iter().zip(a.iter()).any(|(ti, ai)| ti == ai);
    println!("{}", if result { "Yes" } else { "No" });
}
