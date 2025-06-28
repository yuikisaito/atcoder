use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let result = s
        .iter()
        .enumerate()
        .filter(|(i, c)| c.is_uppercase() && *i > 0)
        .map(|(i, _)| i)
        .all(|i| t.contains(&s[i]));

    println!("{}", if result { "Yes" } else { "No" });
}
