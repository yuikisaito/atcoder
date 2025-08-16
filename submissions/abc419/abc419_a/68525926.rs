use proconio::input;

fn main() {
  input! {
    s: String,
  }

  match s.as_ref() {
    "red" => println!("SSS"),
    "blue" => println!("FFF"),
    "green" => println!("MMM"),
    _ => println!("Unknown"),
  }
}
