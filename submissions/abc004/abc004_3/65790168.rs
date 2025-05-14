use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    n %= 60;
    let mut cards: Vec<usize> = (1..=6).collect();
    for i in 0..=n - 1 {
        let i_mod_5 = i % 5;
        cards.swap(i_mod_5, i_mod_5 + 1);
    }
    let result = cards
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("{}", result)
}
