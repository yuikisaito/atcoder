use proconio::input;

fn main() {
    input! {
        n: usize,
        numbers: [usize; n],
    }

    let sum: usize = numbers.iter().sum();
    println!("{}", sum % 100);
}
