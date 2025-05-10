use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut count = 0;
    loop {
        if a.iter().any(|&x| x % 2 != 0) {
            break;
        }
        for x in a.iter_mut() {
            *x /= 2;
        }
        count += 1;
    }

    println!("{}", count);
}
