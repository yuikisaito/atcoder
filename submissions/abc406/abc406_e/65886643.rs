use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [(usize, usize); t],
    }

    for (n, k) in cases {
        println!("{}", {
            let list: Vec<_> = (1..=n)
                .filter(|i| format!("{:b}", i).to_string().matches("1").count() == k)
                .collect();
            let mut sum = 0;
            for i in list {
                sum += i % 998244353;
            }
            sum
        })
    }
}
