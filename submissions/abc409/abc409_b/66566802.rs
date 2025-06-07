use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut crnt_max = 0;
    for i in 0..=*a.iter().max().unwrap() {
        if i <= a.iter().filter(|ai| ai >= &&i).count() {
            crnt_max = i;
        } else {
            break;
        }
    }
    let result = crnt_max;
    println!("{}", result);
}
