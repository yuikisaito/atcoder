use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }

    let mut boxes = vec![0; n];
    let mut result = Vec::<usize>::new();

    for xi in x {
        if xi >= 1 {
            result.push(xi);
            boxes[xi - 1] += 1;
        } else {
            let minsize = boxes.iter().min().unwrap();
            let selected_box = boxes.iter().find_position(|bi| *bi == minsize).unwrap().0;
            boxes[selected_box] += 1;
            result.push(selected_box + 1);
        }
    }
    println!("{}", result.iter().map(|i| format!("{}", i)).collect_vec().join(" "));
}
