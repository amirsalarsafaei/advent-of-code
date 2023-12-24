use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap,HashSet};
use std::process::exit;
use Direction::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    Unknown,
    Left,
    Right,
    Up,
    Down,
}


impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Unknown => Unknown,
            Left => Right,
            Up => Down,
            Down => Up,
            Right => Left,
        }
    }
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State {
    row: usize,
    col: usize,
    last_segment: usize,
    dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.last_segment.cmp(&other.last_segment);
    }
}


impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DX: &'static [i32] = &[0, 0, 1, -1];
const DY: &'static [i32] = &[1, -1, 0, 0];

const DIR: &'static [Direction] = &[Down, Up, Right, Left];

fn main() {
    let table = include_str!("../../input.txt").lines()
        .map(|line| line.chars().map(|c| c as i32 - 48)
            .collect::<Vec<i32>>()).collect::<Vec<_>>();

    let (n, m) = (table.len(), table.first().unwrap().len());

    let mut seen: HashSet<State> = HashSet::new();

    let mut pq: BinaryHeap<(i32, State)> = BinaryHeap::new();
    pq.push(
        (0i32, State { row: 0, col: 0, last_segment: 0, dir: Unknown })
    );


    let is_ok = |x, y| { return min(x, y) >= 0 && x < n as i32 && y < m as i32; };

    while !pq.is_empty() {
        let (dis, node) = pq.pop().unwrap();
        if seen.contains(&node) {
            continue;
        }
        seen.insert(node);

        if node.row == n - 1 && node.col == m - 1 && node.last_segment >= 4 {
            println!("{}", -dis);
            exit(0)
        }

        for i in 0..4 {
            if is_ok(node.row as i32 + DY[i], node.col as i32 + DX[i]) {
                let (x, y) = ((node.row as i32 + DY[i]) as usize,
                              (node.col as i32 + DX[i]) as usize);

                let num = table[x][y];

                if DIR[i] == node.dir {
                    if node.last_segment < 10 {
                        pq.push((
                            dis - num,
                            State {
                                row: x,
                                col: y,
                                last_segment: node.last_segment + 1,
                                dir: node.dir,
                            },
                        ))

                    }
                }else if DIR[i] != node.dir.opposite(){
                    if node.last_segment >= 4 || node.dir == Unknown{
                        pq.push((
                            dis - num,
                            State {
                                row: x,
                                col: y,
                                last_segment: 1,
                                dir: DIR[i],
                            },
                        ))
                    }

                }
            }
        }

    }

}