use advent::prelude::*;
use nom::{bytes::complete::take_while_m_n, character::is_digit, AsBytes, IResult};

/// `get_num` makes use of [nom](https://docs.rs/nom/7.1.3/nom/index.html)
///
/// I really wanted to try it out for this task :D.
/// It consumes a subslice of 1-3 digit characters if it exists,
/// otherwise it returns an error
fn get_num(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while_m_n(1, 3, is_digit)(input)
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut instructions = vec![];

    for (match_start_idx, _) in input.match_indices("mul(") {
        // find pos of `mul(`, from that idx parse with nom
        let take_from_idx = match_start_idx + 4;
        // check if we can retrieve the first number
        if let Ok((remainder, first_num)) = get_num(input[take_from_idx..].as_bytes()) {
            // must be followed up by a comma
            if remainder.first() != Some(&b',') {
                continue;
            }
            // now we can try parsing the second number
            if let Ok((remainder, second_num)) = get_num(remainder[1..].as_bytes()) {
                // which must be follow up by a closing parenthesis
                if remainder.first() != Some(&b')') {
                    continue;
                }

                // we have passed all checks!
                instructions.push((
                    std::str::from_utf8(first_num)
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                    std::str::from_utf8(second_num)
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                ));
            }
        }
    }

    instructions
}

fn default_input() -> Vec<(i64, i64)> {
    parse_input(include_input!(2024 / 03))
}

fn part1(input: Vec<(i64, i64)>) -> i64 {
    input.iter().map(|(n1, n2)| n1 * n2).sum()
}

fn part2(input: Vec<(i64, i64)>) -> i64 {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let reports =
        parse_input("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
    assert_eq!(part1(reports.clone()), 161);
    // assert_eq!(part2(reports), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 161085926);
    // assert_eq!(part2(input), 2);
}
