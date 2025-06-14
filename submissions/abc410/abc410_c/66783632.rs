use itertools::Itertools;
use text_io::read;

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let mut digits = (1..=n).collect_vec();
    for _ in 0..q {
        let input: String = read!("{}\n");
        let mut iter = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let a = iter.next();
        let b = iter.next();
        let c = iter.next();
        match a.unwrap() {
            1 => digits[b.unwrap() - 1] = c.unwrap(),
            2 => println!("{}", digits[b.unwrap() - 1]),
            3 => {
                let cnt = b.unwrap() % n;
                let head = digits.get(0..cnt).unwrap().to_vec();
                digits = digits.get(cnt..).unwrap().to_vec();
                digits.extend(head);
            },
            _ => {}
        }
    }
}
