use proconio::input;

fn main() {
    input! {
        n: u128,
        k: u32,
        a: [u128; n],
    }

    let mut calculator: u128 = 1;
    for i in a {
        calculator = if i >= 10u128.pow(k) {
            1
        } else {
            let result = calculator * i;
            if result >= 10u128.pow(k) {
                1
            } else {
                result
            }
        }
    }

    println!("{}", calculator);
}
