use std::{env, collections::HashSet};

#[derive(Debug, Clone)]
enum OminoState {
    Use,
    Queue,
    Unuse,
}

type OminoStates = Vec<(OminoState, i32, i32)>;

struct GenOmino {
    states: OminoStates,
    set: HashSet<(i32, i32)>,
    output: Vec<Vec<(i32, i32)>>,
}

fn omino_print(states: &[(i32, i32)]) {
        let (mut left, mut right, mut above) = (0, 0, 0);
        for &(x, y) in states {
            left = if x < left { x } else { left };
            right = if x > right { x } else { right };
            above = if y > above { y } else { above };
        }
        let mut grid = vec![vec![' '; (right-left+1) as usize]; (above+1) as usize];
        for &(x, y) in states {
            grid[y as usize][(x-left) as usize] = '#';
        }
        for line in grid.iter().rev() {
            let mut str = String::new();
            for char in line {
                str.push(*char);
            }
            println!("{}", str);
        }
        println!();
}

impl GenOmino {
    fn dfs(&mut self, n: usize) {
        if n == 0 {
            self.output.push(
                self.states.iter().filter_map(
                    |state| if let (OminoState::Use, x, y) = *state { Some((x, y)) } else { None }
                    ).collect()
                );
            return;
        }
        let len = self.states.len();
        for i in 0..len {
            if let (OminoState::Queue, x, y) = self.states[i] {
                self.states[i] = (OminoState::Use, x, y);
                for (dx, dy) in [(0,-1), (-1,0), (1,0), (0,1)] {
                    let (nx, ny) = (x+dx, y+dy);
                    if !self.set.contains(&(nx, ny)) && (ny >= 1 || ny == 0 && nx >= 0)  {
                        self.states.push((OminoState::Queue, nx, ny));
                        self.set.insert((nx, ny));
                    }
                }
                self.dfs(n-1);
                self.states[i] = (OminoState::Unuse, x, y);
                for j in len..self.states.len() {
                    let (_, x, y) = self.states[j];
                    self.set.remove(&(x, y));
                }
                self.states.truncate(len);
                for j in i+1..len {
                    if let (OminoState::Unuse, x, y) = self.states[j] {
                        self.states[j] = (OminoState::Queue, x, y);
                    }
                }
            }
        }
    }

    fn new() -> Self {
        Self {
            output: vec![],
            set: HashSet::from_iter([(0, 0)]),
            states: vec![(OminoState::Queue, 0, 0)],
        }
    }
}

fn reverse_omino(omino: &mut Vec<(i32, i32)>) {
    let mut dx = 0;
    for (x, y) in omino.iter_mut() {
        if *y == 0 && dx < *x { dx = *x; }
    }
    for (x, _) in omino.iter_mut() {
        *x = dx - *x;
    }
    omino.sort();
}

fn rotate_omino(omino: &mut Vec<(i32, i32)>) {
    let (mut dx, mut dy) = (0, 0);
    for (x, _) in omino.iter_mut() {
        if *x < dx { dx = *x; }
    }
    for (x, y) in omino.iter_mut() {
        if *x == dx && dy < *y { dy = *y; }
    }
    for (x, y) in omino.iter_mut() {
        (*x, *y) = (dy - *y, *x - dx);
    }
    omino.sort();
}

fn tidy_ominos(ominos: &mut Vec<Vec<(i32, i32)>>) {
    let mut ret = HashSet::new();
    for omino in ominos.iter_mut() {
        if (|| {
            for _ in 0..4 {
                rotate_omino(omino);
                for _ in 0..2 {
                    reverse_omino(omino);
                    if ret.contains(omino) { return false; }
                }
            }
            true
        })() { ret.insert(omino.clone()); };
    }
    *ominos = ret.into_iter().collect();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum EdgeState {
    Above, Below, Left, Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Edge {
    state: EdgeState,
    pos: i32,
    len: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum NoriPos {
    None, Corner, Edge, Float,
}

fn extract_edges(omino: &[(i32, i32)]) -> HashSet<(i32, i32, EdgeState)> {
    let mut edges = HashSet::new();
    for (x, y) in omino {
        for (state, dx, dy) in [
            (EdgeState::Below, 0, 1),
            (EdgeState::Above, 1, 0),
            (EdgeState::Right, 0, 0),
            (EdgeState::Left , 1, 1),
        ] {
            edges.insert((x+dx, y+dy, state));
        }
    }
    let mut edges_dub = Vec::new();
    for (x, y, state) in edges.iter() {
        match state {
            EdgeState::Above => edges_dub.push((*x  , *y+1, EdgeState::Below)),
            EdgeState::Below => edges_dub.push((*x  , *y-1, EdgeState::Above)),
            EdgeState::Left  => edges_dub.push((*x-1, *y  , EdgeState::Right)),
            EdgeState::Right => edges_dub.push((*x+1, *y  , EdgeState::Left )),
        }
    }
    for edge in edges_dub {
        edges.remove(&edge);
    }
    edges
}

fn nori_pos(omino: &[(i32, i32)]) -> NoriPos {
    let mut edges = extract_edges(omino);
    let mut edge_groups: Vec<Vec<Edge>> = Vec::new();
    while !edges.is_empty() {
        let mut group = Vec::new();
        let mut que = (|| {
            for edge in edges.iter() {
                return Some(*edge);
            }
            unreachable!();
        })();
        edges.remove(&que.unwrap());
        let mut take_next = |x, y| {
            for state in [EdgeState::Above, EdgeState::Below, EdgeState::Left, EdgeState::Right] {
                let opt = edges.take(&(x, y, state));
                if let Some(_) = opt {
                    return opt;
                }
            }
            None
        };
        let (mut tx, mut ty, mut tmp) = que.unwrap();
        let mut cnt = 0;
        while let Some((x, y, state)) = que {
            if tmp == state {
                cnt += 1;
            } else {
                group.push(Edge {
                    state: tmp,
                    pos: match tmp {
                        EdgeState::Left | EdgeState::Right => ty,
                        _ => tx,
                    },
                    len: cnt,
                });
                (tmp, cnt) = (state, 1);
                (tx, ty) = (x, y);
            }
            que = match state {
                EdgeState::Above => take_next(x  , y+1),
                EdgeState::Below => take_next(x  , y-1),
                EdgeState::Left  => take_next(x-1, y  ),
                EdgeState::Right => take_next(x+1, y  ),
            }
        }
        group.push(Edge {
            state: tmp,
            pos: match tmp {
                EdgeState::Left | EdgeState::Right => ty,
                _ => tx,
            },
            len: cnt,
        });
        if group.first().unwrap().state == group.last().unwrap().state {
            group[0].len += group.last().unwrap().len;
            group.pop();
        }
        edge_groups.push(group);
    }
    let mut leftest = 0;
    for groups in edge_groups.iter() {
        for edge in groups.iter() {
            if edge.state == EdgeState::Below && edge.pos < leftest {
                leftest = edge.pos;
            }
        }
    }
    // float, edge, corner, edge2
    let res = edge_groups.iter().fold((true, true, true, true),
        |mut acc, group| {
            for i in 0..group.len() {
                let mut t = (true, true, true, true);
                let Edge { state: s1, pos: p1, len: l1 } = group[i];
                let Edge { state: s2, pos: _, len: l2 } = group[(i+1)%group.len()];
                if s1 == EdgeState::Below && p1 == leftest {
                    t.2 = false; t.3 = false;
                }
                if s1 == EdgeState::Right && p1 == 0 { t.1 = false; t.2 = false; }
                if l1 >= 3 { t.0 = false; }
                match (s1, s2) {
                    (EdgeState::Above, EdgeState::Left)
                        | (EdgeState::Left, EdgeState::Below)
                        | (EdgeState::Below, EdgeState::Right)
                        | (EdgeState::Right, EdgeState::Above)
                        => (),
                    _ => {
                        if l1 == 1 && l2 == 1 { t.0 = false; }
                        if l1 >= 2 && l2 >= 2 { t.0 = false; }
                    },
                }
                acc.0 &= t.0;
                if t.1 { acc.1 &= t.0; }
                if t.2 { acc.2 &= t.0; }
                if t.3 { acc.3 &= t.0; }
            }
            acc
        });
    return match res {
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
    tidy_ominos(&mut ominos.output);
    let mut f = Vec::new();
    let mut e = Vec::new();
    let mut c = Vec::new();
    for omino in ominos.output.iter_mut() {
        let mut state = NoriPos::None;
        for _ in 0..4 {
            rotate_omino(omino);
            let ret = nori_pos(omino);
            if state < ret { state = ret; }
        }
        match state {
            NoriPos::None => (),
            NoriPos::Float => { f.push(omino); }
            NoriPos::Edge => { e.push(omino); }
            NoriPos::Corner => { c.push(omino); }
        }
    }
    for omino in f {
        println!("Float");
        omino_print(&omino);
    }
    for omino in e {
        println!("Edge");
        omino_print(&omino);
    }
    for omino in c {
        println!("Corner");
        omino_print(&omino);
    }
}
