use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        xy: [(usize, usize); n],
    }

    let cnt = xy.iter().filter(|(x, y)| *x <= l && r <= *y).count();

    println!("{}", cnt);
}
