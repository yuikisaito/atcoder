use itertools::Itertools;
use proconio::input;
use text_io::read;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut digits = (1..=n).collect_vec();
    let mut offset = 0;
    for _ in 0..q {
        let input: String = read!("{}\n");
        let mut iter = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let a = iter.next();
        let b = iter.next();
        let c = iter.next();
        match a.unwrap() {
            1 => digits[(b.unwrap() + offset - 1) % n] = c.unwrap(),
            2 => println!("{}", digits[(b.unwrap() + offset - 1) % n]),
            3 => {
                offset = (b.unwrap() + offset) % n;
            },
            _ => {}
        }
    }
}
