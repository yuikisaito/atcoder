use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }
    let result = a.iter().any(|&ai| ai == x);

    println!("{}", if result { "Yes" } else { "No" });
}
