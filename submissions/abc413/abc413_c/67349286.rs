use text_io::read;

fn main() {
    let q: usize = read!();
    let mut items: Vec<(usize, usize)> = Vec::new();

    let mut items_offset = 0;
    let mut inner_offset = 0;
    for _ in 0..q {
        let t: usize = read!();
        match t {
            1 => {
                let c: usize = read!();
                let x: usize = read!();
                items.push((x, c));
            }
            _ => {
                let k: usize = read!();
                let mut cnt = 0;
                let mut deleted_sum = 0;
                while cnt < k {
                    let k_remain = k - cnt;
                    let inner_remain = items[items_offset].1 - inner_offset;
                    if k_remain < inner_remain {
                        deleted_sum += items[items_offset].0 * k_remain;
                        inner_offset += k_remain;
                        break;
                    } else {
                        deleted_sum += items[items_offset].0 * inner_remain;
                        inner_offset = 0;
                        items_offset += 1;
                        cnt += inner_remain;
                    }
                }

                println!("{}", deleted_sum);
            }
        }
    }
}
