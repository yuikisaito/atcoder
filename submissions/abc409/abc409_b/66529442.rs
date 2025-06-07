use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let result = (0..=*a.iter().max().unwrap())
        .map(|i| {
            if i <= a.iter().filter(|ai| ai >= &&i).collect_vec().len() {
                i
            } else {
                0
            }
        })
        .max()
        .unwrap();
    println!("{}", result);
}
