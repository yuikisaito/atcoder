use proconio::input;

fn main() {
    input! {
        n: usize,
        total: usize,
    }

    for x in 0..(n + 1) {
        for y in 0..(n - x + 1) {
            for z in 0..(n - x - y + 1) {
                if (10000 * x + 5000 * y + 1000 * z) == total {
                    println!("{} {} {}", x, y, z);
                    return;
                }
            }
        }
    }
    println!("-1 -1 -1");
}
