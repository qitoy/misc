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

impl GenOmino {
    fn print(states: &[(i32, i32)]) {
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum EdgeState {
    Above, Below, Left, Right,
}

fn is_nori_able(omino: &[(i32, i32)]) -> bool {
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
    {
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
    }
    let mut edge_groups: Vec<Vec<(EdgeState, i32)>> = Vec::new();
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
        let (mut tmp, mut cnt) = (que.unwrap().2, 0);
        while let Some((x, y, state)) = que {
            if tmp == state {
                cnt += 1;
            } else {
                group.push((tmp, cnt));
                (tmp, cnt) = (state, 1);
            }
            que = match state {
                EdgeState::Above => take_next(x  , y+1),
                EdgeState::Below => take_next(x  , y-1),
                EdgeState::Left  => take_next(x-1, y  ),
                EdgeState::Right => take_next(x+1, y  ),
            }
        }
        group.push((tmp, cnt));
        if group.first().unwrap().0 == group.last().unwrap().0 {
            group[0].1 += group.last().unwrap().1;
            group.pop();
        }
        edge_groups.push(group);
    }
    return edge_groups.iter().all(|group| {
        for i in 0..group.len() {
            let (state1, len1) = group[i];
            let (state2, len2) = group[(i+1)%group.len()];
            if len1 >= 3 { return false; }
            match (state1, state2) {
                (EdgeState::Above, EdgeState::Left ) => continue,
                (EdgeState::Left , EdgeState::Below) => continue,
                (EdgeState::Below, EdgeState::Right) => continue,
                (EdgeState::Right, EdgeState::Above) => continue,
                _ => (),
            }
            if len1 == 1 && len2 == 1 { return false; }
            if len1 >= 2 && len2 >= 2 { return false; }
        }
        return true;
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = args[1].parse().unwrap();
    let mut ominos = GenOmino::new();
    ominos.dfs(n);
    for omino in ominos.output {
        if is_nori_able(&omino) {
            GenOmino::print(&omino);
        }
    }
}
