use text_io::read;

fn main() {
  let t: usize = read!();

  for _ in 0..t {
    let n: usize = read!();
    let w: usize = read!();
    let mut c = Vec::<usize>::new();
    for _ in 0..n {
      c.push(read!());
    }

    println!(
      "{}",
      (1..=2 * w)
        .map(|x| {
          c.iter()
            .enumerate()
            .filter(|&(a, &_b)| (a + x) % (2 * w) < w)
            .map(|(_a, &b)| b)
            .sum::<usize>()
        })
        .min()
        .unwrap()
    )
  }
}
