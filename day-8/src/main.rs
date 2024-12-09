use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    let map = input
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut nodes = HashMap::new();
    let nrows = map.len();
    let ncols = map[0].len();
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                nodes
                    .entry(*c)
                    .and_modify(|x: &mut Vec<(usize, usize)>| x.push((i, j)))
                    .or_insert_with(|| vec![(i, j)]);
            }
        }
    }
    let mut antinodes = HashSet::new();
    for nodes in nodes.values() {
        for (cur, (i, j)) in nodes.iter().enumerate() {
            for (ii, jj) in &nodes[cur + 1..] {
                if jj > j {
                    // the second node is on the right
                    if ii > i {
                        // the second node is in the bottom right
                        let ip = ii - i;
                        let jp = jj - j;
                        // bottom right anti node
                        antinodes.insert((ii + ip, jj + jp));
                        // top left anti-node
                        antinodes.insert((i.wrapping_sub(ip), j.wrapping_sub(jp)));
                    } else {
                        // the second node is in the top right
                        let ip = i - ii;
                        let jp = jj - j;
                        // top right anti-node
                        antinodes.insert((ii.wrapping_sub(ip), jj + jp));
                        // bottom left anti-node
                        antinodes.insert((i + ip, j.wrapping_sub(jp)));
                    }
                } else {
                    // the second node is on the left
                    if ii > i {
                        // the second node is in the bottom left
                        let ip = ii - i;
                        let jp = j - jj;
                        // bottom left anti-node
                        antinodes.insert((ii + ip, jj.wrapping_sub(jp)));
                        // top right anti-node
                        antinodes.insert((i.wrapping_sub(ip), j + jp));
                    } else {
                        // the second node is in the top left
                        let ip = ii - i;
                        let jp = j - jj;
                        // top left anti-node
                        antinodes.insert((ii.wrapping_sub(ip), jj.wrapping_sub(jp)));
                        // bottom right anti-node
                        antinodes.insert((i + ip, j + jp));
                    }
                }
            }
        }
    }

    let unique = antinodes
        .iter()
        .filter(|(i, j)| *i < nrows && *j < ncols)
        .collect::<Vec<_>>();
    let mut map = map.clone();
    for (i, j) in &unique {
        map[*i][*j] = '#';
    }
    println!(
        "{}",
        map.iter()
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("Part One: {}", unique.len());

    let mut antinodes = HashSet::new();
    for nodes in nodes.values() {
        for (cur, (i, j)) in nodes.iter().enumerate() {
            for (ii, jj) in &nodes[cur + 1..] {
                antinodes.insert((*i, *j));
                antinodes.insert((*ii, *jj));
                if jj > j {
                    // the second node is on the right
                    if ii > i {
                        // the second node is in the bottom right
                        let ip = ii - i;
                        let jp = jj - j;
                        // bottom right anti node
                        let mut iters = 0;
                        loop {
                            if ii + iters * ip > nrows {
                                break;
                            }
                            if jj + iters * jp > ncols {
                                break;
                            }
                            antinodes.insert((ii + iters * ip, jj + iters * jp));
                            iters += 1;
                        }
                        // top left anti-node
                        let mut iters = 0;
                        loop {
                            if i.wrapping_sub(iters * ip) > nrows {
                                break;
                            }
                            if j.wrapping_sub(iters * jp) > ncols {
                                break;
                            }
                            antinodes
                                .insert((i.wrapping_sub(iters * ip), j.wrapping_sub(iters * jp)));
                            iters += 1;
                        }
                    } else {
                        // the second node is in the top right
                        let ip = i - ii;
                        let jp = jj - j;
                        // top right anti-node
                        let mut iters = 0;
                        loop {
                            if ii.wrapping_sub(iters * ip) > nrows {
                                break;
                            }
                            if jj + iters * jp > ncols {
                                break;
                            }
                            antinodes.insert((ii.wrapping_sub(iters * ip), jj + iters * jp));
                            iters += 1;
                        }
                        // bottom left anti-node
                        let mut iters = 0;
                        loop {
                            if i + iters * ip > nrows {
                                break;
                            }
                            if j.wrapping_sub(iters * jp) > ncols {
                                break;
                            }
                            antinodes.insert((i + iters * ip, j.wrapping_sub(iters * jp)));
                            iters += 1;
                        }
                    }
                } else {
                    // the second node is on the left
                    if ii > i {
                        // the second node is in the bottom left
                        let ip = ii - i;
                        let jp = j - jj;
                        // bottom left anti-node
                        let mut iters = 0;
                        loop {
                            if ii + iters * ip > nrows {
                                break;
                            }
                            if jj.wrapping_sub(iters * jp) > ncols {
                                break;
                            }
                            antinodes.insert((ii + iters * ip, jj.wrapping_sub(iters * jp)));
                            iters += 1;
                        }
                        // top right anti-node
                        let mut iters = 0;
                        loop {
                            if i.wrapping_sub(iters * ip) > nrows {
                                break;
                            }
                            if j + iters * jp > ncols {
                                break;
                            }
                            antinodes.insert((i.wrapping_sub(iters * ip), j + iters * jp));
                            iters += 1;
                        }
                    } else {
                        // the second node is in the top left
                        let ip = ii - i;
                        let jp = j - jj;
                        // top left anti-node
                        let mut iters = 0;
                        loop {
                            if ii.wrapping_sub(iters * ip) > nrows {
                                break;
                            }
                            if jj.wrapping_sub(iters * jp) > ncols {
                                break;
                            }
                            antinodes
                                .insert((ii.wrapping_sub(iters * ip), jj.wrapping_sub(iters * jp)));
                            iters += 1;
                        }
                        // bottom right anti-node
                        let mut iters = 0;
                        loop {
                            if i + iters * ip > nrows {
                                break;
                            }
                            if j + iters * jp > ncols {
                                break;
                            }
                            antinodes.insert((i + iters * ip, j + iters * jp));
                            iters += 1;
                        }
                    }
                }
            }
        }
    }

    let unique = antinodes
        .iter()
        .filter(|(i, j)| *i < nrows && *j < ncols)
        .collect::<Vec<_>>();
    let mut map = map.clone();
    for (i, j) in &unique {
        map[*i][*j] = '#';
    }
    println!(
        "{}",
        map.iter()
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("Part Two: {}", unique.len());
}
