use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n]
    }

    let cnt = ab.iter().filter(|ab| ab[0] < ab[1]).count();
    println!("{}", cnt);
}
