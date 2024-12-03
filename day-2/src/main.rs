#[derive(PartialEq, Eq)]
enum Direction {
    Increasing,
    Decreasing,
}

fn is_report_safe(report: &[i32]) -> bool {
    let direction = if report[0] - report[1] > 0 {
        Direction::Decreasing
    } else {
        Direction::Increasing
    };
    for (prev, cur) in report.iter().zip(&report[1..]) {
        // if decreases but direction is increasing
        if prev - cur > 0 && direction == Direction::Increasing {
            return false;
        // if increases but direction is decreasing
        } else if prev - cur < 0 && direction == Direction::Decreasing {
            return false;
        }
        let diff = (prev - cur).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

fn main() {
    let input = include_str!("input.txt");

    let reports = input
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut safe = 0;
    for report in &reports {
        safe += (is_report_safe(report)) as usize;
    }

    println!("Part One: {}", safe);

    let mut safe_dampened = 0;
    'outer: for report in reports {
        let safe = is_report_safe(&report);
        if safe {
            safe_dampened += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            if is_report_safe(&report) {
                safe_dampened += 1;
                continue 'outer;
            }
        }
    }

    println!("Part Two: {}", safe_dampened);
}
