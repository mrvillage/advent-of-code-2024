fn possible(nums: &[usize]) -> Vec<usize> {
    nums.iter().fold(vec![], |prev, n| {
        if prev.is_empty() {
            return vec![*n];
        }
        prev.iter()
            .map(|x| x + n)
            .chain(prev.iter().map(|x| x * n))
            .collect()
    })
}

fn possible_concat(nums: &[usize]) -> Vec<usize> {
    nums.iter().fold(vec![], |prev, n| {
        if prev.is_empty() {
            return vec![*n];
        }
        prev.iter()
            .map(|x| x + n)
            .chain(prev.iter().map(|x| x * n))
            .chain(
                prev.iter()
                    .map(|x| format!("{x}{n}").parse::<usize>().unwrap()),
            )
            .collect()
    })
}

fn main() {
    let input = include_str!("input.txt");

    let equations = input
        .lines()
        .map(|line| {
            let (res, eq) = line.split_once(": ").unwrap();
            (
                res.parse::<usize>().unwrap(),
                eq.split(' ')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (res, nums) in &equations {
        let pos = possible(nums);
        if pos.contains(res) {
            sum += res;
        }
    }

    println!("Part One: {}", sum);

    let mut sum = 0;
    for (res, nums) in equations {
        let pos = possible_concat(&nums);
        if pos.contains(&res) {
            sum += res;
        }
    }

    println!("Part Two: {}", sum);
}
