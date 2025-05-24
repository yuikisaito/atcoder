use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    let mut result = 0;
    for a in 1..=6 {
        for b in 1..=6 {
            let n: i32 = a - b;
            if a + b >= x || n.abs() >= y {
                result += 1;
            }
        }
    }

    println!("{}", result as f64 / 36.)
}
