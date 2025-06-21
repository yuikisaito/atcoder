use text_io::read;

fn main() {
    let n: usize = read!();
    let q: usize = read!();

    let mut pc = vec![String::new(); n];
    let mut server = String::new();

    (0..q).for_each(|_| {
        let t: usize = read!();
        let p: usize = read!();
        match t {
            1 => {
                pc[p - 1] = server.clone();
            }
            2 => {
                let s: String = read!();
                pc[p - 1].push_str(&s);
            }
            _ => {
                server = pc[p - 1].clone();
            }
        }
    });

    println!("{}", server);
}
