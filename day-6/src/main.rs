use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    use Direction::*;

    let input = include_str!("input.txt");
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut visited = HashSet::new();
    let mut dir = Up;
    let (mut i, mut j) = (0, 0);
    for (ii, row) in map.iter().enumerate() {
        for (jj, col) in row.iter().enumerate() {
            if *col == '^' {
                (i, j) = (ii, jj);
            }
        }
    }
    visited.insert((i, j));
    let nrows = map.len();
    let ncols = map[0].len();
    while i > 0 && i < nrows && j > 0 && j < ncols {
        (i, j) = match dir {
            Up => {
                if map[i - 1][j] == '#' {
                    dir = Right;
                    (i, j)
                } else {
                    (i - 1, j)
                }
            },
            Down => {
                if map[i + 1][j] == '#' {
                    dir = Left;
                    (i, j)
                } else {
                    (i + 1, j)
                }
            },
            Left => {
                if map[i][j - 1] == '#' {
                    dir = Up;
                    (i, j)
                } else {
                    (i, j - 1)
                }
            },
            Right => {
                if map[i][j + 1] == '#' {
                    dir = Down;
                    (i, j)
                } else {
                    (i, j + 1)
                }
            },
        };
        visited.insert((i, j));
    }

    println!("Part One: {}", visited.len());

    let mut new = 0;
    // for ii in 0..nrows {
    // for jj in 0..ncols {
    for (ii, jj) in visited {
        let mut map = map.clone();
        if map[ii][jj] == '^' || map[ii][jj] == '#' {
            continue;
        } else {
            map[ii][jj] = '#';
        }
        let mut visited = HashSet::new();
        let mut dir = Up;
        let (mut i, mut j) = (0, 0);
        for (ii, row) in map.iter().enumerate() {
            for (jj, col) in row.iter().enumerate() {
                if *col == '^' {
                    (i, j) = (ii, jj);
                }
            }
        }
        visited.insert((i, j, dir));
        let nrows = map.len();
        let ncols = map[0].len();
        while i > 0 && i < nrows - 1 && j > 0 && j < ncols - 1 {
            (i, j) = match dir {
                Up => {
                    if map[i - 1][j] == '#' {
                        dir = Right;
                        (i, j)
                    } else {
                        (i - 1, j)
                    }
                },
                Down => {
                    if map[i + 1][j] == '#' {
                        dir = Left;
                        (i, j)
                    } else {
                        (i + 1, j)
                    }
                },
                Left => {
                    if map[i][j - 1] == '#' {
                        dir = Up;
                        (i, j)
                    } else {
                        (i, j - 1)
                    }
                },
                Right => {
                    if map[i][j + 1] == '#' {
                        dir = Down;
                        (i, j)
                    } else {
                        (i, j + 1)
                    }
                },
            };
            if visited.contains(&(i, j, dir)) {
                new += 1;
                break;
            }
            visited.insert((i, j, dir));
        }
    }
    // }

    println!("Part Two: {}", new);
}
