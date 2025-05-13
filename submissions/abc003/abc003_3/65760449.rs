use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut r: [usize; n]
    }

    r.sort();
    println!(
        "{:.6}",
        &r[n - k..]
            .iter()
            .fold(0., |acc, &x| (acc as f64 + x as f64) / 2.)
    )
}
