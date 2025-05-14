use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
    }

    let r1 = (r - 1) / 2;
    let r2 = (r - 1) - r1;
    let r_count: usize = (1..=r1).chain(1..=r2).sum();

    let g1 = (g - 1) / 2;
    let g2 = (g - 1) - g1;
    let g_count: usize = (1..=g1).chain(1..=g2).sum();

    let b1 = (b - 1) / 2;
    let b2 = (b - 1) - b1;
    let b_count: usize = (1..=b1).chain(1..=b2).sum();

    println!("{}", r_count + g_count + b_count)
}
