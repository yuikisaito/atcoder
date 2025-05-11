use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    let mut count = 0;
    for s in 0..=100 {
        if a <= s && s < b && c <= s && s < d {
            count += 1;
        }
    }
    println!("{}", count)
}
