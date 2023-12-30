use nap::{extract_edges, omino_print, rotate_omino, tidy_ominos, Edge, EdgeState, GenOmino};
use std::env;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum NoriPos {
    None,
    Corner,
    Edge,
    Float,
}

fn nori_pos(omino: &[(i32, i32)]) -> NoriPos {
    let edge_groups = extract_edges(omino);
    let mut leftest = 0;
    for groups in edge_groups.iter() {
        for edge in groups.iter() {
            if edge.state == EdgeState::Below && edge.pos < leftest {
                leftest = edge.pos;
            }
        }
    }
    // float, edge, corner, edge2
    let res = edge_groups
        .iter()
        .fold((true, true, true, true), |mut acc, group| {
            for i in 0..group.len() {
                let mut t = (true, true, true, true);
                let Edge {
                    state: s1,
                    pos: p1,
                    len: l1,
                } = group[i];
                let Edge {
                    state: s2,
                    pos: _,
                    len: l2,
                } = group[(i + 1) % group.len()];
                if s1 == EdgeState::Below && p1 == leftest {
                    t.2 = false;
                    t.3 = false;
                }
                if s1 == EdgeState::Right && p1 == 0 {
                    t.1 = false;
                    t.2 = false;
                }
                if l1 >= 3 {
                    t.0 = false;
                }
                match (s1, s2) {
                    (EdgeState::Above, EdgeState::Left)
                    | (EdgeState::Left, EdgeState::Below)
                    | (EdgeState::Below, EdgeState::Right)
                    | (EdgeState::Right, EdgeState::Above) => (),
                    _ => {
                        if l1 == 1 && l2 == 1 {
                            t.0 = false;
                        }
                        if l1 >= 2 && l2 >= 2 {
                            t.0 = false;
                        }
                    }
                }
                acc.0 &= t.0;
                if t.1 {
                    acc.1 &= t.0;
                }
                if t.2 {
                    acc.2 &= t.0;
                }
                if t.3 {
                    acc.3 &= t.0;
                }
            }
            acc
        });
    match res {
        (false, false, true, true) => NoriPos::None,
        (false, false, false, _) => NoriPos::None,
        (false, false, true, _) => NoriPos::Corner,
        (false, true, _, _) => NoriPos::Edge,
        (true, _, _, _) => NoriPos::Float,
    }
}

fn main() {
    #![allow(unreachable_code)]
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let mut ominos = GenOmino::new();
    ominos.dfs(n);
    let mut ominos = ominos.output;
    tidy_ominos(&mut ominos);
    let mut f = Vec::new();
    let mut e = Vec::new();
    let mut c = Vec::new();
    for omino in ominos.iter_mut() {
        let mut state = NoriPos::None;
        for _ in 0..4 {
            rotate_omino(omino);
            let ret = nori_pos(omino);
            if state < ret {
                state = ret;
            }
        }
        match state {
            NoriPos::None => (),
            NoriPos::Float => {
                f.push(omino);
            }
            NoriPos::Edge => {
                e.push(omino);
            }
            NoriPos::Corner => {
                c.push(omino);
            }
        }
    }
    for omino in f {
        println!("Float");
        omino_print(omino);
    }
    for omino in e {
        println!("Edge");
        omino_print(omino);
    }
    for omino in c {
        println!("Corner");
        omino_print(omino);
    }
}
