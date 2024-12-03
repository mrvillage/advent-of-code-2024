fn main() {
    let input = include_str!("input.txt");

    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        left.push(l.trim().parse::<i32>().unwrap());
        right.push(r.trim().parse::<i32>().unwrap());
    }
    let mut left_sorted = left.clone();
    left_sorted.sort();
    let mut right_sorted = right.clone();
    right_sorted.sort();
    let diff = left_sorted
        .iter()
        .zip(right_sorted)
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("Part One: {}", diff);

    let mut sim = 0;
    for i in left_sorted {
        sim += i as usize * right.iter().filter(|x| **x == i).count();
    }

    println!("Part Two: {}", sim);
}
