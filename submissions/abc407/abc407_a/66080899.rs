use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let n = (a as f64 / b as f64).round();

    println!("{}", n)
}
