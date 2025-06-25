extern crate union_find;
use union_find::prelude::*;

fn main() {
    let edges = vec![
        Edge { u: 0, v: 1, w: 5 },
        Edge { u: 0, v: 2, w: 4 },
        Edge { u: 1, v: 2, w: 3 },
    ];

    println!("MST: {:?}", kruskal(3, edges));
}

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
    w: usize,
}

fn kruskal(nodes: usize, mut edges: Vec<Edge>) -> Vec<Edge> {
    edges.sort_by(|e1, e2| e1.w.cmp(&e2.w));

    let mut uf = UnionFind::new(nodes);

    let mut mst = Vec::new();

    for e in edges {
        if !uf.in_same(e.u, e.v) {
            uf.union_elements(e.u, e.v);
            mst.push(e);
        }
    }

    mst
}
