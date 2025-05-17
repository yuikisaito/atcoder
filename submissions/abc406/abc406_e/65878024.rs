use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [(usize, usize); t],
    }

    for (n, k) in cases {
        println!(
            "{}",
            (1..=n)
                .filter(|i| format!("{:b}", i).to_string().matches("1").count() == k)
                .sum::<usize>()
                % 998244353
        )
    }
}
