use advent::prelude::*;

// because advent::part() does not account for tuples(?)
type Lists = (Vec<i64>, Vec<i64>);

fn parse_input(input: &str) -> Lists {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for l in input.split('\n') {
        if l == "" {
            continue;
        }
        let mut both = l.split_whitespace();
        list1.push(both.next().unwrap().parse::<i64>().unwrap());
        list2.push(both.next().unwrap().parse::<i64>().unwrap());
    }

    list1.sort();
    list2.sort();
    (list1, list2)
}

fn default_input() -> Lists {
    parse_input(include_input!(2024 / 01))
}

fn part1(lists: Lists) -> i64 {
    let mut total = 0i64;
    let (list1, list2) = lists;

    for i in 0..list1.len() {
        total += (list1[i] - list2[i]).abs();
    }

    total
}

fn part2(lists: Lists) -> i64 {
    let mut total = 0i64;
    let (list1, list2) = lists;

    // "dumb" method
    for a in &list1 {
        let mut times_found = 0i64;

        for b in &list2 {
            if a == b {
                times_found += 1;
            }
        }

        total += times_found * a;
    }

    total
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let lists = parse_input(
        "3   4
4   3
2   5
1   3
3   9
3   3
",
    );
    assert_eq!(part1(lists.clone()), 11);
    assert_eq!(part2(lists), 31);
}

#[test]
fn default() {
    let lists = default_input();
    assert_eq!(part1(lists.clone()), 936063);
    assert_eq!(part2(lists), 23150395);
}
