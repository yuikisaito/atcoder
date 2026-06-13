use itertools::izip;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    matrix: [[Usize1]; n],
  }

  let mut recieved = vec![Vec::new(); n];
  for (recievers, sender) in izip!(matrix, 0..n) {
    println!("{:?}", recievers);
    for i in recievers {
      recieved[i].push(sender);
    }
  }

  for senders in recieved {
    println!("{} {}", senders.len(), senders.into_iter().map(|n| (n + 1).to_string()).join(" "));
  }
}
