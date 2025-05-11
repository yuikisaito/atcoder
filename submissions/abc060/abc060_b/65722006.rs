use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        _: usize,
    }

    if a == b {
        println!("YES")
    } else {
        println!("NO")
    }
}
