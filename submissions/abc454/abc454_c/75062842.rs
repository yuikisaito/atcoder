use petgraph::graph::DiGraph;
use petgraph::visit::Dfs;
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    ab: [(u32, u32); m],
  }

  let mut g = DiGraph::<usize, ()>::new();
  for _ in 0..n {
    g.add_node(0);
  }
  for (a, b) in ab {
    g.add_edge((a - 1).into(), (b - 1).into(), ());
  }
  let mut dfs = Dfs::new(&g, 0.into());
  while let Some(nx) = dfs.next(&g) {
    g[nx] += 1;
  }

  println!("{}", g.raw_nodes().into_iter().filter(|node| node.weight > 0).count());
}
