use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
    }

    let mut count = 0;
    for blue in 1..=n {
        for red in 1..=n {
            if blue + red <= s {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
