pub(crate) fn solve() {
    println!("Day 3");
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    let part_one: u32 = input.lines()
        .map(find_relevant_char)
        .map(char_priority)
        .sum();
    println!("Day 3 Part One: {}", part_one);

    let part_two: u32 = input.lines()
        .collect::<Vec<_>>()
        .chunks(3)

        .map(|chunk| find_relevant_char2(chunk.to_vec()))
        .map(char_priority)
        .sum();
    println!("Day 3 Part Two: {}", part_two);
}

fn find_relevant_char(s: &str) -> char {
    let halves = s.split_at(s.len() / 2);
    for char in halves.0.chars() {
        if halves.1.contains(char) {
            return char;
        }
    }
    panic!("Did not find any duplicate char in both string halves")
}

fn find_relevant_char2(lines: Vec<&str>) -> char {
    let first = lines[0];
    let second = lines[1];
    let third = lines[2];

    first.chars()
        .find(|c| second.contains(*c) && third.contains(*c))
        .unwrap()
}

mod tests {
    use crate::day3::find_relevant_char;

    #[test]
    fn finds_relevant_char() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let char: char = find_relevant_char(input);
        assert_eq!('p', char);

        let input = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let char: char = find_relevant_char(input);
        assert_eq!('L', char);
    }

    #[test]
    fn finds_common_char_across_multiple_lines() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];
        let char = super::find_relevant_char2(input);
        assert_eq!('r', char);
    }

    #[test]
    fn calculates_points_for_char() {
        let priority = super::char_priority('p');
        assert_eq!(16, priority);
        let priority = super::char_priority('L');
        assert_eq!(38, priority);
        let priority = super::char_priority('P');
        assert_eq!(42, priority);
        let priority = super::char_priority('t');
        assert_eq!(20, priority);
    }
}

pub(crate) fn char_priority(c: char) -> u32 {
    println!("Char value for {} is {}", c, c as u32);
    return match c.is_lowercase() {
        true => (c as u32) - 96,
        false => (c as u32) - 38
    };
    panic!("Given character is not alpha")
}
