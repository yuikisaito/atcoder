use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut n = 0;
    for i in 1..=b {
        n += a.pow(i as u32);
        if n % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO")
}
