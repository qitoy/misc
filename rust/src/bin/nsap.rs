use nap::{extract_edges, omino_print, tidy_ominos, Edge, EdgeState, GenOmino};
use std::env;

fn nsap(edges: &[Vec<Edge>]) -> bool {
    edges.iter().all(|edges| {
        let mut edges2 = edges.clone();
        edges2.rotate_left(1);
        std::iter::zip(edges, &edges2).all(|(e1, e2)| {
            let Edge {
                state: s1,
                pos: _,
                len: l1,
            } = e1;
            let Edge {
                state: s2,
                pos: _,
                len: l2,
            } = e2;
            match (s1, s2) {
                (EdgeState::Above, EdgeState::Left)
                | (EdgeState::Left, EdgeState::Below)
                | (EdgeState::Below, EdgeState::Right)
                | (EdgeState::Right, EdgeState::Above) => true,
                _ => l1 == l2,
            }
        })
    })
}

fn main() {
    #![allow(unreachable_code)]
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let mut ominos = GenOmino::new();
    ominos.dfs(n);
    let mut ominos = ominos.output;
    tidy_ominos(&mut ominos);

    for omino in ominos {
        let edges = extract_edges(&omino);
        // eprintln!("{edges:?}");
        if nsap(&edges) {
            omino_print(&omino);
        }
    }
}
