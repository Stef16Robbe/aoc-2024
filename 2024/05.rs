use advent::prelude::*;

type PageInfo = (Vec<Vec<i64>>, Vec<Vec<i64>>);

fn parse_input(input: &str) -> PageInfo {
    let parts = input
        .strip_suffix("\n")
        .unwrap()
        .splitn(2, "\n\n")
        .collect::<Vec<_>>();

    let order_rules = parts[0]
        .split("\n")
        .map(|fr| {
            fr.split('|')
                .map(|r| r.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let to_produce = parts[1]
        .split("\n")
        .map(|fp| {
            fp.split(',')
                .map(|p| p.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (order_rules, to_produce)
}

fn default_input() -> PageInfo {
    parse_input(include_input!(2024 / 05))
}

fn part1(input: PageInfo) -> i64 {
    todo!("part 1")
}

fn part2(input: PageInfo) -> i64 {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let reports = parse_input(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
    );
    assert_eq!(part1(reports.clone()), 143);
    // assert_eq!(part2(reports), 4);
}

// #[test]
// fn default() {
//     let input = default_input();
//     assert_eq!(part1(input.clone()), 1);
//     assert_eq!(part2(input), 2);
// }
