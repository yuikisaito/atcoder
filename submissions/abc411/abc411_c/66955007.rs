use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; q],
    }

    let mut masu = vec![false; n];

    a.iter().for_each(|&x| {
        let i = x - 1;
        masu[i] = !masu[i];

        let mut cnt = 0;

        masu.iter()
            .enumerate()
            .filter(|(_, &c)| c)
            .fold(n + 1, |acc, (i, _)| {
                if acc + 1 != i {
                    cnt += 1;
                }
                i
            });

        println!("{}", cnt);
    });
}
