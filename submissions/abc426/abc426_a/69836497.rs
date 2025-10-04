use proconio::input;

fn main() {
  input! {
    x: String,
    y: String,
  }

  let conv = |i: String| -> usize {
    match i.as_str() {
      "Ocelot" => 0,
      "Serval" => 1,
      "Lynx" => 2,
      _ => unreachable!(),
    }
  };

  println!("{}", if conv(x) >= conv(y) { "Yes" } else { "No" });
}
