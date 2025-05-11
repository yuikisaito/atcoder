use proconio::input;

fn main() {
    input! {
        n: usize,
        total: usize,
    }

    let mut maisu = 0;
    for x in 0..(n + 1) {
        maisu = x;
        for y in 0..(n - maisu + 1) {
            maisu += y;
            for z in 0..(n - maisu + 1) {
                if (10000 * x + 5000 * y + 1000 * z) == total {
                    println!("{} {} {}", x, y, z);
                    return;
                }
            }
        }
    }
    println!("-1 -1 -1");
}
