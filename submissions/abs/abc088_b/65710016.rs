use proconio::input;

fn main() {
    input! {
        n: usize,
        mut cards: [u32; n],
    }

    cards.sort();
    let mut alice = 0;
    let mut bob = 0;
    let mut tern = true;
    for i in (0..n).rev() {
        if tern == true {
            alice += cards[i];
            tern = false;
        } else {
            bob += cards[i];
            tern = true;
        }
    }
    println!("{}", alice - bob);
}
