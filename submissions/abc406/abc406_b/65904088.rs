use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        a: [usize; n],
    }

    let mut calculator = 1;
    for i in a {
        calculator = if i >= 10usize.pow(k) {
            1
        } else {
            let result = calculator * i;
            if result >= 10usize.pow(k) {
                1
            } else {
                result
            }
        }
    }

    println!("{}", calculator);
}
