use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut cards: Vec<usize> = (1..=6).collect();
    for i in 0..=n - 1 {
        cards.swap(i % 5, i % 5 + 1);
    }
    let result = cards
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", result)
}
