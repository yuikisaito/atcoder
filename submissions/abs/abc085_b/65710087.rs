use proconio::input;

fn main() {
    input! {
        n: usize,
        mut mochi: [u32; n],
    }

    mochi.sort();
    mochi.dedup();

    println!("{}", mochi.len())
}
