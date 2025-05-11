use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    for k in 1..=b {
        if (k * a) % b == c {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
