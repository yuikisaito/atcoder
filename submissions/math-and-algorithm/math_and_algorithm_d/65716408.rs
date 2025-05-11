use proconio::input;

fn main() {
    input! {
        numbers: [usize; 3],
    }

    let product: usize = numbers.iter().product();
    println!("{}", product);
}
