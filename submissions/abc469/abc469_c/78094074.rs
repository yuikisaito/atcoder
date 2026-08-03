use std::io;
use std::io::Read;
use std::io::Write;

fn main() {
  let mut buffer = Vec::new();
  io::stdin().read_to_end(&mut buffer).unwrap();

  if buffer.is_empty() {
    return;
  }

  let mut ptr = buffer.as_ptr();
  let end_ptr = unsafe { ptr.add(buffer.len()) };

  let mut n: usize = 0;
  unsafe {
    while ptr < end_ptr && *ptr <= b' ' {
      ptr = ptr.add(1);
    }
    while ptr < end_ptr && *ptr > b' ' {
      n = n * 10 + (*ptr - b'0') as usize;
      ptr = ptr.add(1);
    }
    while ptr < end_ptr && *ptr <= b' ' {
      ptr = ptr.add(1);
    }
  }

  let s_ptr = ptr;

  let stdout = io::stdout();
  let mut handle = io::BufWriter::with_capacity(1 << 18, stdout.lock());

  let mut f: usize = 0;

  for _ in 0..n {
    unsafe {
      while f < n && *s_ptr.add(f) == b'o' {
        f += 1;
      }
    }
    f += 1;

    let val = if f < n { f } else { n };
    write_usize(&mut handle, val);
  }
}

#[inline(always)]
fn write_usize<W: Write>(
  writer: &mut W,
  mut val: usize,
) {
  if val == 0 {
    let _ = writer.write_all(b"0\n");
    return;
  }
  let mut buf = [0u8; 20];
  let mut curr = 20;

  curr -= 1;
  buf[curr] = b'\n';

  while val > 0 {
    curr -= 1;
    buf[curr] = (val % 10) as u8 + b'0';
    val /= 10;
  }

  unsafe {
    let ptr = buf.as_ptr().add(curr);
    let len = 20 - curr;
    let slice = std::slice::from_raw_parts(ptr, len);
    let _ = writer.write_all(slice);
  }
}
