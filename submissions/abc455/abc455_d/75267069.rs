use az::Az;
use petgraph::graph::DiGraph;
use petgraph::visit::Dfs;
use petgraph::Direction::Incoming;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    q: usize,
    cp: [(Usize1, Usize1); q],
  }

  let mut g = DiGraph::<(), ()>::new();
  for _ in 0..n {
    g.add_node(());
  }
  for (c, p) in cp {
    let mut neighbors = g.neighbors_directed(c.az::<u32>().into(), Incoming).detach();
    while let Some(e) = neighbors.next_edge(&g) {
      g.remove_edge(e);
    }
    g.add_edge(p.az::<u32>().into(), c.az::<u32>().into(), ());
  }

  for i in 0..n {
    let idx = i.az::<u32>().into();
    println!(
      "{}",
      if g.neighbors_directed(idx, Incoming).count() == 0 {
        let mut dfs = Dfs::new(&g, idx);
        let mut cnt = 0;
        while dfs.next(&g).is_some() {
          cnt += 1;
        }
        cnt
      } else {
        0
      }
    );
  }
}
