use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};

pub(crate) fn solve() {
    let input = std::fs::read_to_string("../input/day5.txt").unwrap();
    let mut lines = input.lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    let mut piles = Piles(transpose_rev(crate_lines));
    println!("{piles:?}");

    assert!(lines.next().unwrap().is_empty());

    let instructions = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect::<Vec<_>>();

    for instruction in instructions {
        println!("{instruction:?}");
        piles.execute2(instruction);
        print!("{piles:?}");
    }
    piles.print_top_elements();
}

#[derive(PartialEq, Copy, Clone)]
struct Crate(char);

struct Piles(Vec<Vec<Crate>>);

impl Debug for Piles {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (index, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", index, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn execute(&mut self, instruction: Instruction) {
        for _ in 0..instruction.amount {
            let krate = self.0[instruction.from].pop().unwrap();
            self.0[instruction.to].push(krate);
        }
    }

    fn execute2(&mut self, ins: Instruction) {
        let [src, dst] = self
            .0
            .get_many_mut([ins.from, ins.to])
            .expect("out of bounds / overlapping src/dst stacks");

        dst.extend(src.drain((src.len() - ins.amount)..))
    }

    fn print_top_elements(&self) {
        println!(
            "top elements = {}",
            self.0.iter().map(|pile| pile.last().unwrap()).join("")
        )
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

// Not actually my own solution. This one was a touch too tricky to figure out on my own, in a
// language / ecosystem I am not (yet) familiar with. Props go to @fasterthanlime.
// https://fasterthanli.me/series/advent-of-code-2022/part-5
#[cfg(test)]
mod tests {
    use crate::day5::{
        parse_crate, parse_crate_line, parse_crate_or_hole, parse_digit, parse_hole,
        parse_instruction, parse_pile_number, Crate, Instruction,
    };

    #[test]
    fn parses_crate() {
        let a = "[A]";
        let result_a = parse_crate(a);
        assert!(result_a.is_ok());
        let crate_a = result_a.unwrap().1;
        assert_eq!(crate_a, Crate('A'));

        let b = "[B] [C]";
        let result_b = parse_crate(b);
        assert!(result_b.is_ok());
        let unwrapped_b = result_b.unwrap();
        let crate_b = unwrapped_b.1;
        assert_eq!(crate_b, Crate('B'));
        assert_eq!(unwrapped_b.0, " [C]");
    }

    #[test]
    fn parses_hole() {
        let hole = "   ";
        let result = parse_hole(hole).unwrap().1;
        assert_eq!(result, ());
    }

    #[test]
    fn parses_crate_or_hole() {
        let hole = "   ";
        let krate = "[C]";

        let hole_result = parse_crate_or_hole(hole).unwrap().1;
        assert!(hole_result.is_none());
        let krate_result = parse_crate_or_hole(krate).unwrap().1;
        assert!(krate_result.is_some());
    }

    #[test]
    fn parses_crate_line() {
        let input = "[A] [B]    ";
        let result: Vec<Option<Crate>> = parse_crate_line(input).unwrap().1;
        assert_eq!(result, vec![Some(Crate('A')), Some(Crate('B')), None]);
    }

    #[test]
    fn parses_digit() {
        let ok_input = "3";
        let result = parse_digit(ok_input).unwrap().1;
        assert_eq!(result, 3);
    }

    #[test]
    fn handles_error_parsing_digit() {
        let nok_input = "A";
        let result = parse_digit(nok_input);
        assert!(result.is_err());
    }

    #[test]
    fn parses_pile_number() {
        let input = "2";
        let result = parse_pile_number(input).unwrap().1;
        assert_eq!(result, 1);
    }

    #[test]
    fn parses_instruction() {
        let input = "move 3 from 1 to 2";
        let result = parse_instruction(input).unwrap().1;
        assert_eq!(
            result,
            Instruction {
                from: 0,
                to: 1,
                amount: 3,
            }
        );
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_digit),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(amount, from, to)| Instruction { amount, from, to },
    )(input)
}

fn parse_pile_number(input: &str) -> IResult<&str, usize> {
    map(parse_digit, |i| i - 1)(input)
}

fn parse_digit(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_crate_line(input: &str) -> IResult<&str, Vec<Option<Crate>>> {
    // Parse first crate in line
    let (mut input, parse_result) = parse_crate_or_hole(input)?;
    let mut line = vec![parse_result];

    // Parse rest of line
    loop {
        let (remainder, maybe_crate) = opt(preceded(tag(" "), parse_crate_or_hole))(input)?;
        match maybe_crate {
            Some(krate) => line.push(krate),
            None => break,
        }
        input = remainder;
    }
    Ok((input, line))
}

fn parse_crate_or_hole(input: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(input)
}

fn parse_crate(input: &str) -> IResult<&str, Crate> {
    let factory = |s: &str| Crate(s.chars().next().unwrap());
    let crate_parser = delimited(tag("["), take(1_usize), tag("]"));
    map(crate_parser, factory)(input)
}

fn parse_hole(input: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(input)
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

impl Debug for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
