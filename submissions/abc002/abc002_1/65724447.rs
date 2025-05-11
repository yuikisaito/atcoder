use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }

    println!("{}", std::cmp::max(x, y));
}
