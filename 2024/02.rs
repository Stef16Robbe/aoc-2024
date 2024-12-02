use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut input = input.to_owned();
    input.pop();
    let mut reports: Vec<Vec<i64>> = vec![];

    for r in input.split("\n") {
        reports.push(r.split(" ").map(|n| n.parse::<i64>().unwrap()).collect());
    }

    reports
}

fn default_input() -> Vec<Vec<i64>> {
    parse_input(include_input!(2024 / 02))
}

fn part1(input: Vec<Vec<i64>>) -> i64 {
    let mut total_safe = 0;

    for report in input {
        let mut safe = true;

        for i in 1..report.len() {
            let diff = (report[i - 1] - report[i]).abs();

            if !((1..=3).contains(&diff)) {
                safe = false;
            }
        }

        if !(report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1])) {
            safe = false;
        }

        if safe {
            total_safe += 1;
        }
    }

    total_safe
}

fn part2(input: Vec<Vec<i64>>) -> i64 {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let reports = parse_input(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
    );
    assert_eq!(part1(reports.clone()), 2);
    // assert_eq!(part2(reports), 31);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 631);
    // assert_eq!(part2(input), 2);
}