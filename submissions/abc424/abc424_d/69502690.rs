use itertools::Itertools;
use text_io::read;

fn main() {
  let t: usize = read!();
  for _ in 0..t {
    let h: usize = read!();
    let w: usize = read!();
    let mut s = Vec::<Vec<char>>::new();
    for _ in 0..h {
      let line: String = read!();
      s.push(line.chars().collect_vec());
    }

    let mut matrix = vec![0; w * h];
    for x in 0..w - 1 {
      for y in 0..h - 1 {
        if s[y][x] == '#' && s[y + 1][x] == '#' && s[y][x + 1] == '#' && s[y + 1][x + 1] == '#' {
          matrix[y * w + x] += 1;
          matrix[(y + 1) * w + x] += 1;
          matrix[y * w + (x + 1)] += 1;
          matrix[(y + 1) * w + (x + 1)] += 1;
        }
      }
    }

    let mut cnt = 0;
    loop {
      if matrix.iter().max().unwrap() == &0 {
        break;
      }
      let i = matrix.iter().position_max().unwrap();
      let x = i % w;
      let y = i / w;
      if y > 0 {
        matrix[(y - 1) * w + x] = 0;
        if x < w {
          matrix[(y - 1) * w + (x + 1)] = 0;
        }
        if x > 0 {
          matrix[(y - 1) * w + (x - 1)] = 0;
        }
      }
      if y < h {
        matrix[(y + 1) * w + x] = 0;
        if x > 0 {
          matrix[(y + 1) * w + (x - 1)] = 0;
        }
        if x < w {
          matrix[(y + 1) * w + (x + 1)] = 0;
        }
      }
      if x > 0 {
        matrix[y * w + (x - 1)] = 0;
      }
      if x < w {
        matrix[y * w + (x + 1)] = 0;
      }
      matrix[y * w + x] = 0;
      cnt += 1;
    }
    println!("{}", cnt);
  }
}
