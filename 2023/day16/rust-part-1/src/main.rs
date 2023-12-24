use std::cmp::min;
use std::collections::HashSet;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use Direction::*;

#[derive(PartialEq, EnumIter, Clone, Copy, Hash, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Hash, Clone, Copy, Eq)]
struct Move {
    i: i32,
    j: i32,
    dir: Direction,
}

impl Direction {
    fn d_point(&self) -> (i32, i32) {
        return match self {
            Left => (0, -1),
            Right => (0, 1),
            Up => (-1, 0),
            Down => (1, 0),
        };
    }
}

fn main() {
    let input = include_str!("../../input");
    let table = input.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (n, m) = (table.len(), table.first().unwrap().len());

    let mut seen: HashSet<Move> = HashSet::new();
    dfs(Move{i: 0, j: 0, dir: Right}, &table, &mut seen);

    let mut cnt = 0i32;
    for i in 0..n {
        for j in 0..m {
            let mut flag = false;
            for dir in Direction::iter() {
                if seen.contains(&Move{i: i as i32, j: j as i32, dir}) {
                    flag = true;
                    break;
                }
            }
            if flag {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt)
}

fn dfs(mv: Move, table: &Vec<Vec<char>>, seen: &mut HashSet<Move>) {
    if seen.contains(&mv) {
        return;
    }

    let (n, m) = (table.len(), table.first().unwrap().len());
    if mv.i >= n as i32 || mv.j >= m as i32 || min(mv.i, mv.j) < 0 {
        return;
    }
    seen.insert(mv);

    let ch = table[mv.i as usize][mv.j as usize];
    match ch {
        '|' => {
            if mv.dir == Up || mv.dir == Down {
                let d_point = mv.dir.d_point();
                return dfs(
                    Move { i: mv.i + d_point.0, j: mv.j + d_point.1, dir: mv.dir },
                    table, seen,
                );
            }
            dfs(Move { i: mv.i + 1, j: mv.j, dir: Down }, table, seen);
            dfs(Move { i: mv.i - 1, j: mv.j, dir: Up }, table, seen);
        }
        '-' => {
            if mv.dir == Left || mv.dir == Right {
                let d_point = mv.dir.d_point();
                return dfs(
                    Move { i: mv.i + d_point.0, j: mv.j + d_point.1, dir: mv.dir },
                    table, seen,
                );
            }
            dfs(Move { i: mv.i, j: mv.j + 1, dir: Right }, table, seen);
            dfs(Move { i: mv.i, j: mv.j - 1, dir: Left }, table, seen);
        }
        '/' => {
            match mv.dir {
                Up => {
                    dfs(Move { i: mv.i, j: mv.j + 1, dir: Right }, table, seen);
                }
                Down => {
                    dfs(Move { i: mv.i, j: mv.j - 1, dir: Left }, table, seen);
                }
                Left => {
                    dfs(Move { i: mv.i + 1, j: mv.j, dir: Down }, table, seen);
                }
                Right => {
                    dfs(Move { i: mv.i - 1, j: mv.j, dir: Up }, table, seen);
                }
            }
        }
        '\\' => {
            match mv.dir {
                Up => {
                    dfs(Move { i: mv.i, j: mv.j - 1, dir: Left }, table, seen);
                }
                Down => {
                    dfs(Move { i: mv.i, j: mv.j + 1, dir: Right }, table, seen);
                }
                Left => {
                    dfs(Move { i: mv.i - 1, j: mv.j, dir: Up }, table, seen);
                }
                Right => {
                    dfs(Move { i: mv.i + 1, j: mv.j, dir: Down }, table, seen);
                }
            }
        }
        '.' => {
            let d_point = mv.dir.d_point();
            return dfs(
                Move { i: mv.i + d_point.0, j: mv.j + d_point.1, dir: mv.dir },
                table, seen,
            );
        }
        _ => {}
    }
}
