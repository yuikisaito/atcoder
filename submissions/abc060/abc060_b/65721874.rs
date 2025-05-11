use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut n = 0;
    for i in 1..10 ^ 9 {
        n += a * i;
        if n % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
