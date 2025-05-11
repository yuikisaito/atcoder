use proconio::input;

fn main() {
    input! {
        n: usize,
        data: [String; n],
    }

    let mut sorted: Vec<(&str, usize)> = Vec::new();
    for i in &data {
        let mut split = i.splitn(2, '-');
        let s = split.next().unwrap();
        let sn: usize = s.parse().unwrap();
        sorted.push((i, sn));
    }
    sorted.sort_by_key(|row| row.1);
    for i in sorted {
        println!("{}", i.0)
    }
}
