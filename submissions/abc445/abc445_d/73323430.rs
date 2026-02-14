use proconio::input;

fn main() {
  input! {
    big_h: usize,
    big_w: usize,
    n: usize,
    mut pieces: [(usize, usize); n],
  }

  let mut crnt_size = (big_h, big_w);
  let mut positions = vec![None; n];
  for _ in 0..n {
    let pos = pieces.iter().position(|&(h, w)| w == crnt_size.1 || h == crnt_size.0).unwrap();
    positions[pos] = Some((big_h - crnt_size.0, big_w - crnt_size.1));
    let piece = pieces[pos];
    pieces[pos] = (0, 0);
    if crnt_size.0 == piece.0 {
      crnt_size.1 -= piece.1
    } else {
      crnt_size.0 -= piece.0
    }
  }
  for l in 0..n {
    let piece = positions[l].unwrap();
    println!("{} {}", piece.0, piece.1);
  }
}
