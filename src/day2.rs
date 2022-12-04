use std::str::FromStr;

use crate::day2::Shape::{Paper, Rock, Scissors};

pub(crate) fn solve() {
    let input = std::fs::read_to_string("input/day2.txt").unwrap();
    let part_one_points: i32 = input
        .lines()
        .map(Game::part_one_game_from_str)
        .filter_map(|r| r.ok())
        .map(|g| g.points_for_game())
        .sum();
    println!("Part One: {}", part_one_points);

    let part_two_points: i32 = input
        .lines()
        .map(Game::part_two_game_from_str)
        .filter_map(|r| r.ok())
        .map(|g| g.points_for_game())
        .sum();
    println!("Part Two: {}", part_two_points);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
struct ElfPlay {
    shape: Shape,
}

#[derive(PartialEq, Debug)]
struct MyPlay {
    shape: Shape,
}

#[derive(PartialEq, Debug)]
struct Game {
    elf_play: ElfPlay,
    my_play: MyPlay,
}

impl Game {
    pub(crate) fn points_for_game(&self) -> i32 {
        let points_for_game = match (&self.my_play.shape, &self.elf_play.shape) {
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Paper, Scissors) => 0,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6,
            _ => 3,
        };
        points_for_game + self.my_play.points_for_shape()
    }

    fn part_one_game_from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() != 3 {
            return Err("Given string has incorrect length");
        }
        let vec = s.split(' ').collect::<Vec<_>>();
        let elf_play = ElfPlay::from_str(vec[0]).unwrap();
        let my_play = MyPlay::from_str(vec[1]).unwrap();
        Ok(Game { elf_play, my_play })
    }

    fn part_two_game_from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() != 3 {
            return Err("Given string has incorrect length");
        }
        let vec = s.split_whitespace().collect::<Vec<_>>();
        let elf_play = ElfPlay::from_str(vec[0]).unwrap();
        let my_play = Game::my_play_for(elf_play.shape, vec[1]).unwrap();
        Ok(Game { elf_play, my_play })
    }

    fn my_play_for(elf_shape: Shape, strategy: &str) -> Result<MyPlay, &'static str> {
        match strategy {
            "Y" => Ok(MyPlay { shape: elf_shape }),
            "X" => {
                let my_shape = match elf_shape {
                    Rock => Scissors,
                    Paper => Rock,
                    Scissors => Paper,
                };
                Ok(MyPlay { shape: my_shape })
            }
            "Z" => {
                let my_shape = match elf_shape {
                    Rock => Paper,
                    Paper => Scissors,
                    Scissors => Rock,
                };
                Ok(MyPlay { shape: my_shape })
            }
            _ => Err("Can't deduce MyPlay for given opposing shape and strategy"),
        }
    }
}

impl MyPlay {
    fn points_for_shape(&self) -> i32 {
        match self.shape {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl FromStr for MyPlay {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MyPlay { shape: Rock }),
            "Y" => Ok(MyPlay { shape: Paper }),
            "Z" => Ok(MyPlay { shape: Scissors }),
            _ => Err(()),
        }
    }
}

impl FromStr for ElfPlay {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(ElfPlay { shape: Rock }),
            "B" => Ok(ElfPlay { shape: Paper }),
            "C" => Ok(ElfPlay { shape: Scissors }),
            _ => Err(()),
        }
    }
}

#[test]
fn creates_play_from_string() {
    let a = ElfPlay::from_str("A").unwrap();
    assert_eq!(ElfPlay { shape: Rock }, a);
    let b = ElfPlay::from_str("B").unwrap();
    assert_eq!(ElfPlay { shape: Paper }, b);
    let c = ElfPlay::from_str("C").unwrap();
    assert_eq!(ElfPlay { shape: Scissors }, c);
    let x = MyPlay::from_str("X").unwrap();
    assert_eq!(MyPlay { shape: Rock }, x);
    let y = MyPlay::from_str("Y").unwrap();
    assert_eq!(MyPlay { shape: Paper }, y);
    let z = MyPlay::from_str("Z").unwrap();
    assert_eq!(MyPlay { shape: Scissors }, z);
}

#[test]
fn creates_part_one_game_from_string() {
    let game = Game::part_one_game_from_str("B Y").unwrap();
    assert_eq!(
        Game {
            elf_play: ElfPlay { shape: Paper },
            my_play: MyPlay { shape: Paper }
        },
        game
    );
}

#[test]
fn creates_part_two_game_from_string() {
    let game = Game::part_two_game_from_str("A Y").unwrap();
    assert_eq!(
        Game {
            elf_play: ElfPlay { shape: Rock },
            my_play: MyPlay { shape: Rock }
        },
        game
    );

    assert_eq!(4, game.points_for_game());

    let game = Game::part_two_game_from_str("B X").unwrap();
    assert_eq!(
        Game {
            elf_play: ElfPlay { shape: Paper },
            my_play: MyPlay { shape: Rock }
        },
        game
    );
}

#[test]
fn adds_plays_for_points() {
    let elf_play = ElfPlay::from_str("A").unwrap();
    let my_play = MyPlay::from_str("Y").unwrap();
    let game = Game { elf_play, my_play };
    let points = game.points_for_game();
    assert_eq!(8, points);
}

#[test]
fn points_for_my_play() {
    let rock = MyPlay { shape: Rock };
    assert_eq!(1, rock.points_for_shape());
    let paper = MyPlay { shape: Paper };
    assert_eq!(2, paper.points_for_shape());
    let scissors = MyPlay { shape: Scissors };
    assert_eq!(3, scissors.points_for_shape());
}
