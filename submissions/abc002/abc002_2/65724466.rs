use proconio::input;
use regex::Regex;

fn main() {
    input! {
        w: String,
    }

    let re = Regex::new("a|i|u|e|o").unwrap();
    let result = re.replace_all(&w, "");
    println!("{}", result);
}
