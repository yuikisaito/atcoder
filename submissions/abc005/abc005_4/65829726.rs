use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [[usize; n]; n],
        q: usize,
        p: [usize; q],
    }

    let d_ref = &d;
    p.iter().for_each(|&pi| {
        // pi の約数から (h, w) の候補を作る
        let factors = (1..=pi)
            .flat_map(|h| {
                // w は 1 から (pi / h)
                // h * w <= pi が保証
                (1..=pi / h).map(move |w| (h, w))
            })
            .filter(|(h, w)| *h <= n && *w <= n);

        let result = factors
            .map(|(h, w)| {
                (0..=n - h)
                    .flat_map(|x| {
                        (0..=n - w).map(move |y| {
                            (0..h)
                                .flat_map(move |di| (0..w).map(move |dj| d_ref[x + di][y + dj]))
                                .sum::<usize>()
                        })
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();
        println!("{}", result)
    });
}
