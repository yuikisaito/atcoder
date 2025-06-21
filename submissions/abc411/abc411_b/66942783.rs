use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n-1]
    }

    let mut pos = vec![0usize];

    d.iter().fold(0, |acc, i| {
        let sum = acc + i;
        pos.push(sum);
        sum
    });

    (0..n - 1).for_each(|x| {
        let mut x_pos = Vec::<String>::new();
        (x + 1..n).for_each(|y| {
            x_pos.push((pos[y] - pos[x]).to_string());
        });
        println!("{}", x_pos.join(" "))
    });
}
