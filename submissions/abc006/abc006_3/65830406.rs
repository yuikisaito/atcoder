use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for adult in 1..=n {
        for elderly in 1..=n - adult {
            let baby = n - adult - elderly;
            let legs = 2 * adult + 3 * elderly + 4 * baby;
            if legs == m {
                println!("{} {} {}", adult, elderly, baby);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
