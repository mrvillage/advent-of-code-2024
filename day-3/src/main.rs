fn main() {
    let input = include_str!("input.txt");

    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for (_, [one, two]) in re.captures_iter(input).map(|x| x.extract()) {
        sum += one.parse::<i32>().unwrap() * two.parse::<i32>().unwrap();
    }

    println!("Part One: {}", sum);

    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)\(\)|(d)(o)n't\(\)").unwrap();
    let mut sum = 0;
    let mut d = true;
    for (m, [one, two]) in re.captures_iter(input).map(|x| x.extract()) {
        if m == "do()" {
            d = true;
        } else if m == "don't()" {
            d = false;
        } else if d {
            sum += one.parse::<i32>().unwrap() * two.parse::<i32>().unwrap();
        }
    }

    println!("Part Two: {}", sum);
}
