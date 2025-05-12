use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", ((n as f64 - 1.) / 2. + 1.) * 10000.)
}
