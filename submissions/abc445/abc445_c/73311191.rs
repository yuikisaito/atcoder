use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut last = vec![None; n];

  for i in 0..n {
    if let Some(_) = last[i] {
      continue;
    }

    let mut points = Vec::new();
    let mut destination = i;
    points.push(destination);
    loop {
      if let Some(point) = last[destination] {
        destination = point;
        break;
      } else {
        if destination == a[destination] - 1 {
          break
        } else {
          destination = a[destination] - 1;
        }
        points.push(destination);
      }
    }
    for p in points {
      last[p] = Some(destination);
    }
  }

  println!("{}", last.into_iter().map(Option::unwrap).map(|n| n + 1).join(" "));
}
