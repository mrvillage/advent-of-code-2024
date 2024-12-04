const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const REV_XMAS: [char; 4] = ['S', 'A', 'M', 'X'];
const MAS: [char; 3] = ['M', 'A', 'S'];
const REV_MAS: [char; 3] = ['S', 'A', 'M'];

fn main() {
    let input = include_str!("input.txt");

    let mat = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut times = 0;
    let nrows = mat.len();
    let ncols = mat[0].len();

    // check each row for horizontal forwards or backwards
    for r in mat.iter() {
        for i in 0..(ncols - 3) {
            if r[i..i + 4] == XMAS {
                times += 1;
            }
            if r[i..i + 4] == REV_XMAS {
                times += 1;
            }
        }
    }

    // check each column for vertical forwards or backwards
    for c in 0..ncols {
        for r in 0..(nrows - 3) {
            let xmas = [mat[r][c], mat[r + 1][c], mat[r + 2][c], mat[r + 3][c]];
            if xmas == XMAS || xmas == REV_XMAS {
                times += 1;
            }
        }
    }

    // check for diagonals going to the right
    for r in 0..(nrows - 3) {
        for c in 0..(ncols - 3) {
            let xmas = [
                mat[r][c],
                mat[r + 1][c + 1],
                mat[r + 2][c + 2],
                mat[r + 3][c + 3],
            ];
            if xmas == XMAS || xmas == REV_XMAS {
                times += 1;
            }
        }
    }
    // check for diagonals going to the left
    for r in 0..(nrows - 3) {
        for c in 3..(ncols) {
            let xmas = [
                mat[r][c],
                mat[r + 1][c - 1],
                mat[r + 2][c - 2],
                mat[r + 3][c - 3],
            ];
            if xmas == XMAS || xmas == REV_XMAS {
                times += 1;
            }
        }
    }

    println!("Part One: {}", times);

    let mut times = 0;

    for r in 1..(nrows - 1) {
        for c in 1..(ncols - 1) {
            let left = [mat[r - 1][c - 1], mat[r][c], mat[r + 1][c + 1]];
            let right = [mat[r + 1][c - 1], mat[r][c], mat[r - 1][c + 1]];
            if (left == MAS || left == REV_MAS) && (right == MAS || right == REV_MAS) {
                times += 1;
            }
        }
    }

    println!("Part Two: {}", times);
}
