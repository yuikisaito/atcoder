use proconio::input;

fn main() {
    input! {
        n: usize,
        cl: [(char, usize); n],
    }

    if cl.iter().map(|(_, l)| l).sum::<usize>() > 100 {
        println!("Too Long");
        return;
    }

    let mut s = String::new();
    cl.iter().for_each(|(c, l)| {
        for _ in 0..*l {
            s.push(*c);
        }
    });
    println!("{}", s);
}
