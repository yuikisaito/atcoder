use text_io::read;

fn main() {
  let t: usize = read!();

  for _ in 0..t {
    let n: usize = read!();
    let d: usize = read!();
    let mut stocks = Vec::new();

    for _ in 0..n {
      let a: usize = read!();
      stocks.push(a);
    }

    let stocks_sum: usize = stocks.iter().sum();
    let mut stocks_iter = stocks.into_iter();
    let mut used = 0;
    let mut should_trash = 0;

    for i in 0..n {
      let b: usize = read!();
      used += b;

      if i >= d {
        should_trash += stocks_iter.next().unwrap();
        used = used.max(should_trash);
      }
    }

    println!("{}", stocks_sum - used);
  }
}
