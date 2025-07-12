use proconio::input;

fn main() {
    input! {
        n: usize,
        cl: [(char, usize); n],
    }

    if cl.len() > 100 {
        too_long();
        return;
    }
    let mut sum = 0;
    for (_, l) in cl.iter() {
        sum += l;
        if sum > 100 {
            too_long();
            return;
        }
    }

    let mut s = String::new();
    cl.iter().for_each(|(c, l)| {
        for _ in 0..*l {
            s.push(*c);
        }
    });
    println!("{}", s);
}

fn too_long() {
    println!("Too Long");
}
