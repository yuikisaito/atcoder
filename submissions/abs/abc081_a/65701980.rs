use proconio::input;

fn main() {
    input! {
        i: String,
    }
    let num = i.chars().filter(|c| c == "1").count();
    println!("{}", num);
}
