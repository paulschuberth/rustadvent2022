use camino::Utf8PathBuf;
use nom::{
    bytes::complete::tag, bytes::complete::take_while1, combinator::map, sequence::preceeded,
    IResult, branch::alt
};

#[cfg(test)]
mod test {

    use super::*;
}

fn parse_path(input: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(input)
}

#[derive(Debug)]
struct Ls;

#[derive(Debug)]
struct Cd(Utf8PathBuf);

enum Command {
    Ls(Ls),
    Cd(Cd),
}

fn parse_cd(input: &str) -> IResult<&str, Cd> {
    map(preceeded(tag("cd "), parse_path), Cd)(input)
}

fn parse_ls(input: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((map(parse_ls, Command::Ls), map(parse_cd, Command::Cd)))(input)
}
