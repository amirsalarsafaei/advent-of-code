use std::cmp::min;
use std::collections::HashSet;
use std::process::exit;
use strum::IntoEnumIterator;
use Move::*;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
enum Move {
    Left,
    Right,
    Up,
    Down
}

impl Move  {
    fn next_move(&self, a: &char) -> Option<Move> {
        return match a {
            &'7' => {
                if self == &Right {
                    Some(Down)
                } else if self == &Up {
                    Some(Left)
                } else {
                    None
                }
            },
            &'J' => {
                if self == &Right {
                    Some(Up)
                } else if self == &Down {
                    Some(Left)
                } else {
                    None
                }
            },
            &'F' => {
                if self == &Left {
                    Some(Down)
                } else if self == &Up {
                    Some(Right)
                } else {
                    None
                }
            },
            &'L' => {
                if self == &Left {
                    Some(Up)
                } else if self == &Down {
                    Some(Right)
                } else {
                    None
                }
            },
            &'|' => {
                if self == &Up {
                    Some(Up)
                } else if self == &Down {
                    Some(Down)
                } else {
                    None
                }
            },
            &'-' => {
                if self == &Right {
                    Some(Right)
                } else if self == &Left {
                    Some(Left)
                } else {
                    None
                }
            }
            &'S' => {
                Some(self.clone())
            }
            &_ => {None}
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Up => (-1, 0),
            Down=> (1, 0),
            Right => (0, 1),
            Left=> (0, -1)
        }
    }
}

fn find_loop((i,j) : (i32, i32), (n,m) : (usize, usize), mv: Move, seen: &mut HashSet<(i32, i32)>,
             table: &Vec<Vec<char>>, distance: i32) -> Option<i32> {
    if min(i, j) < 0 || i >= n as i32 || j >= m as i32 {
        return None
    }
    let char = table.get(i as usize).unwrap().get(j as usize).unwrap();
    seen.insert((i, j));

    if char == &'S' {
        return Some(distance)
    }
    let next_mv = mv.next_move(&char)?;
    let delta = next_mv.delta();
    let next_loc = (i + delta.0, j + delta.1);
    if !seen.contains(&next_loc) {
        return find_loop(next_loc, (n, m), next_mv, seen, table, distance + 1)
    }
    return None
}


fn main() {
    let input = include_str!("../input.txt");
    let table = input.lines().map(|a| a.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let (n, m) = (table.len(), table.first().unwrap().len());


    for (i, row) in table.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
             if ch == &'S' {
                 for mv in Move::iter() {

                     let res = find_loop(
                         (i as i32 + mv.delta().0, j as i32 + mv.delta().1),
                         (n, m),
                            mv,
                         &mut HashSet::new(),
                         &table,
                         1i32
                     );
                     if let Some(val) = res{
                         println!("{}", val/2);
                         exit(0)
                     }
                 }
             }
        }
    }
}
