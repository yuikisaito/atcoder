use proconio::input;
use regex::Regex;

fn main() {
    input! {
    s: String,
    }

    let re = Regex::new(r"^(?:dreamer|dream|eraser|erase)+$").unwrap();

    if re.is_match(&s) {
        println!("YES");
    } else {
        println!("NO");
    }
}
