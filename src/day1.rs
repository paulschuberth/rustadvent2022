use std::ops::Range;

#[cfg(test)]
mod tests {
    use crate::day1::parse_for_elves;

    use super::sum_calories_per_elf;

    #[test]
    fn sums_strings() {
        let input = vec!["100", "200", "300"];
        let actual = sum_calories_per_elf(input);
        assert_eq!(600, actual)
    }

    #[test]
    fn splits_file_into_vec_of_vecs() {
        let input = std::fs::read_to_string("input/day1-test.txt").unwrap();
        let actual = parse_for_elves(input.as_str());
        assert_eq!(
            vec![vec!["100", "200"], vec!["300", "400", "500"]],
            actual
        )
    }
}

pub(crate) fn solve() {
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let per_elves = parse_for_elves(input.as_str());
    let mut calories_per_elf = per_elves.iter()
        .map(|x1| sum_calories_per_elf(x1.to_vec()))
        .collect::<Vec<i32>>();
    calories_per_elf.sort();
    calories_per_elf.reverse();

    let part_one = calories_per_elf.first().unwrap();
    let part_two = &calories_per_elf[0..3];

    println!("Day 1 part one {}", part_one);
    println!("Day 1 part two {:?}", part_two);
}

fn sum_calories_per_elf(vec: Vec<&str>) -> i32 {
    vec.iter()
        .map(|line| line.parse::<i32>())
        .filter_map(|e| e.ok())
        .sum()
}

fn parse_for_elves(string: &str) -> Vec<Vec<&str>> {
    let elves = string.split("\n\n")
        .collect::<Vec<_>>();
    let calories_per_elf = elves.iter()
        .map(|x| x.split('\n')
            .collect::<Vec<_>>())
        .collect();
    calories_per_elf
}
