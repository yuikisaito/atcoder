use proconio::input;

fn main() {
    input! {
        n: usize,
        numbers: [usize; n]
    }

    println!("{}", numbers.iter().sum());
}
