use proconio::input;

fn main() {
    input! {
        n: u32, a: u32, b: u32,
    }

    let mut num = 0;
    for i in 0..n {
        let keta = i.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
        if a <= keta && keta <= b {
            num += 1;
        }
    }
    print!("{}", num);
}
