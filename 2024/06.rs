use advent::prelude::*;
use std::{collections::HashSet, str::FromStr};

type Position = (usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::Up),
            ">" => Ok(Direction::Right),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            _ => Err(ParseDirectionError),
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    current_pos: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Map {
    bounds: (usize, usize),
    obstacles: HashSet<Position>,
    player: Player,
}

fn parse_input(input: &str) -> Map {
    let rows = input
        .strip_suffix('\n')
        .unwrap_or(input)
        .lines()
        .collect::<Vec<_>>();
    let bounds = (rows[0].len(), rows.len());

    let mut obstacles = HashSet::new();
    let mut current_pos = (0, 0);
    let mut direction = Direction::Up;

    for (row, s) in rows.iter().enumerate() {
        for (col, c) in s.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((col, row));
                }
                '^' | 'v' | '<' | '>' => {
                    current_pos = (col, row);
                    direction = Direction::from_str(c.to_string().as_str()).unwrap();
                }
                _ => {}
            }
        }
    }

    Map {
        bounds,
        obstacles,
        player: Player {
            current_pos,
            direction,
        },
    }
}

fn default_input() -> Map {
    parse_input(include_input!(2024 / 06))
}

fn part1(input: Map) -> i64 {
    todo!("part 1")
}

fn part2(input: Map) -> i64 {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let reports = parse_input(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
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
