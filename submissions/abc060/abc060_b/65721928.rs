use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let mut n = 0;
    let mut i = 0;
    loop {
        n += a * i;
        if n > b {
            break;
        }
        if n % b == c {
            println!("YES");
            return;
        }
        i += 1;
    }
    println!("NO");
}
