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
                    if inner_offset >= items[items_offset].1 {
                        items_offset += 1;
                        inner_offset = 0;
                    }
                    deleted_sum += items[items_offset].0;
                    cnt += 1;
                    inner_offset += 1;
                }
                println!("{}", deleted_sum);
            }
        }
    }
}
