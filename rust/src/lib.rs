use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum OminoState {
    Use,
    Queue,
    Unuse,
}

type OminoStates = Vec<(OminoState, i32, i32)>;

pub struct GenOmino {
    pub states: OminoStates,
    pub set: HashSet<(i32, i32)>,
    pub output: Vec<Vec<(i32, i32)>>,
}

pub fn omino_print(states: &[(i32, i32)]) {
    let (mut left, mut right, mut above) = (0, 0, 0);
    for &(x, y) in states {
        left = if x < left { x } else { left };
        right = if x > right { x } else { right };
        above = if y > above { y } else { above };
    }
    let mut grid = vec![vec![' '; (right - left + 1) as usize]; (above + 1) as usize];
    for &(x, y) in states {
        grid[y as usize][(x - left) as usize] = '#';
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
    pub fn dfs(&mut self, n: usize) {
        if n == 0 {
            self.output.push(
                self.states
                    .iter()
                    .filter_map(|state| {
                        if let (OminoState::Use, x, y) = *state {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
            return;
        }
        let len = self.states.len();
        for i in 0..len {
            if let (OminoState::Queue, x, y) = self.states[i] {
                self.states[i] = (OminoState::Use, x, y);
                for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if !self.set.contains(&(nx, ny)) && (ny >= 1 || ny == 0 && nx >= 0) {
                        self.states.push((OminoState::Queue, nx, ny));
                        self.set.insert((nx, ny));
                    }
                }
                self.dfs(n - 1);
                self.states[i] = (OminoState::Unuse, x, y);
                for j in len..self.states.len() {
                    let (_, x, y) = self.states[j];
                    self.set.remove(&(x, y));
                }
                self.states.truncate(len);
                for j in i + 1..len {
                    if let (OminoState::Unuse, x, y) = self.states[j] {
                        self.states[j] = (OminoState::Queue, x, y);
                    }
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            output: vec![],
            set: HashSet::from_iter([(0, 0)]),
            states: vec![(OminoState::Queue, 0, 0)],
        }
    }
}

pub fn reverse_omino(omino: &mut [(i32, i32)]) {
    let mut dx = 0;
    for (x, y) in omino.iter_mut() {
        if *y == 0 && dx < *x {
            dx = *x;
        }
    }
    for (x, _) in omino.iter_mut() {
        *x = dx - *x;
    }
    omino.sort();
}

pub fn rotate_omino(omino: &mut [(i32, i32)]) {
    let (mut dx, mut dy) = (0, 0);
    for (x, _) in omino.iter_mut() {
        if *x < dx {
            dx = *x;
        }
    }
    for (x, y) in omino.iter_mut() {
        if *x == dx && dy < *y {
            dy = *y;
        }
    }
    for (x, y) in omino.iter_mut() {
        (*x, *y) = (dy - *y, *x - dx);
    }
    omino.sort();
}

pub fn tidy_ominos(ominos: &mut Vec<Vec<(i32, i32)>>) {
    let mut ret = HashSet::new();
    for omino in ominos.iter_mut() {
        if (|| {
            for _ in 0..4 {
                rotate_omino(omino);
                for _ in 0..2 {
                    reverse_omino(omino);
                    if ret.contains(omino) {
                        return false;
                    }
                }
            }
            true
        })() {
            ret.insert(omino.clone());
        };
    }
    *ominos = ret.into_iter().collect();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum EdgeState {
    Above,
    Below,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Edge {
    pub state: EdgeState,
    pub pos: i32,
    pub len: u32,
}

fn omino_edgestate(omino: &[(i32, i32)]) -> HashSet<(i32, i32, EdgeState)> {
    let mut edges = HashSet::new();
    for (x, y) in omino {
        for (state, dx, dy) in [
            (EdgeState::Below, 0, 1),
            (EdgeState::Above, 1, 0),
            (EdgeState::Right, 0, 0),
            (EdgeState::Left, 1, 1),
        ] {
            edges.insert((x + dx, y + dy, state));
        }
    }
    let mut edges_dub = Vec::new();
    for (x, y, state) in edges.iter() {
        match state {
            EdgeState::Above => edges_dub.push((*x, *y + 1, EdgeState::Below)),
            EdgeState::Below => edges_dub.push((*x, *y - 1, EdgeState::Above)),
            EdgeState::Left => edges_dub.push((*x - 1, *y, EdgeState::Right)),
            EdgeState::Right => edges_dub.push((*x + 1, *y, EdgeState::Left)),
        }
    }
    for edge in edges_dub {
        edges.remove(&edge);
    }
    edges
}

pub fn extract_edges(omino: &[(i32, i32)]) -> Vec<Vec<Edge>> {
    let mut edges = omino_edgestate(omino);
    let mut edge_groups: Vec<Vec<Edge>> = Vec::new();
    while !edges.is_empty() {
        let mut group = Vec::new();
        let mut que = edges.iter().next().copied();
        edges.remove(&que.unwrap());
        let mut take_next = |x, y| {
            for state in [
                EdgeState::Above,
                EdgeState::Below,
                EdgeState::Left,
                EdgeState::Right,
            ] {
                let opt = edges.take(&(x, y, state));
                if opt.is_some() {
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
                EdgeState::Above => take_next(x, y + 1),
                EdgeState::Below => take_next(x, y - 1),
                EdgeState::Left => take_next(x - 1, y),
                EdgeState::Right => take_next(x + 1, y),
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
    edge_groups
}
