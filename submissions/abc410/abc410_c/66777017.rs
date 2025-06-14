use itertools::Itertools;
use proconio::{input};
use whiteread::parse_line;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut digits = (1..=n).collect_vec();
    for _ in 0..q {
        let (a, b, c): (Option<usize>, Option<usize>, Option<usize>) = parse_line().unwrap();
        match a.unwrap() {
            1 => digits[b.unwrap() - 1] = c.unwrap(),
            2 => println!("{}", digits[b.unwrap() - 1]),
            3 => {
                let head = digits.get(0..b.unwrap()).unwrap().to_vec();
                digits = digits.get(b.unwrap()..).unwrap().to_vec();
                digits.extend(head);
            },
            _ => {}
        }
    }
}
