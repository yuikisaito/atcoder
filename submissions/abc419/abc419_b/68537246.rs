use text_io::read;

fn main() {
  let mut balls = vec![];
  for _ in 0..read!() {
    let t: usize = read!();
    match t {
      1 => {
        let x: usize = read!();
        balls.push(x);
      },
      2 => {
        let min = balls.iter().min().unwrap();
        println!("{}", min);
        if let Some(i) = balls.iter().position(|i| i == min) {
          balls.remove(i);
        }
      },
      _ => {},
    }
  }
}
