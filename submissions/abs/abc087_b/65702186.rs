use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }

    let mut num = 0;
    for o in 0..a + 1 {
        for p in 0..b + 1 {
            for q in 0..c + 1 {
                if 500 * o + 100 * p + 50 * q == x {
                    num += 1;
                }
            }
        }
    }
    print!("{}", num)
}
