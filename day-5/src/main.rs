fn correctly_ordered(print: &[usize], rules: &[(usize, usize)]) -> Option<(usize, usize)> {
    for (f, s) in rules {
        if let (Some(f), Some(s)) = (
            print.iter().position(|x| *x == *f),
            print.iter().position(|x| *x == *s),
        ) {
            if f > s {
                return Some((f, s));
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("input.txt");

    let mut rules = Vec::new();
    let mut lines = input.lines();
    for line in &mut lines {
        if line.trim() == "" {
            break;
        }
        let (l, r) = line.split_once('|').unwrap();
        rules.push((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()));
    }

    let mut prints = Vec::new();
    for line in lines {
        prints.push(
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut sum = 0;
    let mut bad = Vec::new();
    'print: for (i, print) in prints.iter().enumerate() {
        for (f, s) in &rules {
            if let (Some(f), Some(s)) = (
                print.iter().position(|x| *x == *f),
                print.iter().position(|x| *x == *s),
            ) {
                if f > s {
                    bad.push(i);
                    continue 'print;
                }
            }
        }
        sum += print[print.len() / 2];
    }

    println!("Part One: {}", sum);

    let mut sum = 0;
    for i in bad {
        let print = &mut prints[i];
        while let Some((f, s)) = correctly_ordered(print, &rules) {
            print.swap(f, s);
        }
        sum += print[print.len() / 2];
    }

    println!("Part Two: {}", sum);
}
