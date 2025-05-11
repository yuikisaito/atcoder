use proconio::{input, marker::Chars};

fn main() {
    // 入力読み込み
    input! {
        h: usize,
        w: usize,
        grid: [Chars; h],
    }

    // 方向ベクトル (上, 下, 左, 右)
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 全セルを確認
    for i in 0..h {
        for j in 0..w {
            // 黒マスのみ考慮
            if grid[i][j] != '#' {
                continue;
            }

            // 黒マスの隣に黒マスがあるか調べる
            let mut has_neighbor = false;
            for &(di, dj) in &directions {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                    continue;
                }
                if grid[ni as usize][nj as usize] == '#' {
                    has_neighbor = true;
                    break;
                }
            }

            // 隣接黒マスがなければ不可能
            if !has_neighbor {
                println!("No");
                return;
            }
        }
    }

    // 条件を全て満たす場合
    println!("Yes");
}
