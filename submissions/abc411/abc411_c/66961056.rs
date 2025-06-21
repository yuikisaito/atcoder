use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; q],
    }

    let mut change = Vec::<usize>::new();

    a.iter().for_each(|&x| {
        let mut cnt = 0;
        if let Some(ri) = change.iter().position(|&s| s == x) {
            change.remove(ri);
        } else {
            change.push(x);
        }
        change.sort();
        change.iter().fold(n + 1, |acc, &y| {
            if acc + 1 != y {
                cnt += 1;
            }
            y
        });
        println!("{}", cnt);
    });
}
