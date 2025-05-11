use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
    }

    let mut cnt = 0;
    for n in a..=b {
        if n % x == 0 {
            cnt += 1;
        }
    }
    println!("{}", cnt)
}
