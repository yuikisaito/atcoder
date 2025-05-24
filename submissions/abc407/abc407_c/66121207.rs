use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt = 0;
    let mut digits = s.iter().map(|c| c.to_digit(10).unwrap()).collect_vec();
    while !(digits.len() == 1 && digits[0] == 0) {
        if *digits.last().unwrap() > 0 {
            digits.iter_mut().for_each(|d| {
                if *d == 0 {
                    *d = 9;
                } else {
                    *d -= 1;
                }
            });
            cnt += 1;
        } else {
            digits.remove(digits.len() - 1);
            cnt += 1;
        }
    }
    cnt += 1;

    println!("{}", cnt);
}
