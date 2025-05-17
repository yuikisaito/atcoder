use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut calculator = 1;
    for i in a {
        let result = calculator * i;
        calculator = if result.to_string().len() > k {
            1
        } else {
            result
        }
    }

    println!("{}", calculator);
}
