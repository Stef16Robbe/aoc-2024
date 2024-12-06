use advent::prelude::*;
use std::{collections::HashSet, str::FromStr};

// X, Y
type Position = (usize, usize);

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
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
    let bounds = (rows[0].len() - 1, rows.len() - 1);

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

fn part1(mut map: Map) -> usize {
    let mut total_pos: HashSet<(usize, usize)> = HashSet::new();

    while map.player.current_pos.0 <= map.bounds.0 && map.player.current_pos.1 <= map.bounds.1 {
        total_pos.insert(map.player.current_pos);
        let new_pos = match map.player.direction {
            // All Y coordinate moves are switched, because the Y axis is actually reversed
            // (started counting from the top)
            Direction::Up => (map.player.current_pos.0, map.player.current_pos.1 - 1),
            Direction::Right => (map.player.current_pos.0 + 1, map.player.current_pos.1),
            Direction::Down => (map.player.current_pos.0, map.player.current_pos.1 + 1),
            Direction::Left => (map.player.current_pos.0 - 1, map.player.current_pos.1),
        };

        if map.obstacles.contains(&new_pos) {
            map.player.direction.turn()
        } else {
            map.player.current_pos = new_pos;
        }
    }

    total_pos.len()
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
    assert_eq!(part1(reports.clone()), 41);
    // assert_eq!(part2(reports), 4);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    // assert_eq!(part2(input), 2);
}
