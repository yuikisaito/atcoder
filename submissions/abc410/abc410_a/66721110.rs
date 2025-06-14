use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        k: usize,
    }

    let cnt = a.iter().filter(|ai| k <= **ai).count();

    println!("{}", cnt);
}
