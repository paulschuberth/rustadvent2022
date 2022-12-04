use std::collections::HashSet;
use std::hash::Hash;

pub(crate) fn solve() {
    let input = std::fs::read_to_string("input/day4.txt").unwrap();
    let part_one = input
        .lines()
        .map(parse_ranges)
        .map(one_tuple_component_completely_contains_the_other)
        .filter(|x| *x)
        .count();

    println!("Day 4 Part One: {}", part_one);

    let part_two = input
        .lines()
        .map(parse_ranges)
        .map(tuple_components_overlap)
        .filter(|x| *x)
        .count();

    println!("Day 4 Part Two: {}", part_two);
}

#[cfg(test)]
mod tests {
    use crate::day4::one_tuple_component_completely_contains_the_other;
    use crate::day4::parse_ranges;
    use crate::day4::tuple_components_overlap;
    use std::collections::HashSet;

    #[test]
    fn splits_str_into_tuple() {
        let input = "2-4,6-8";
        let sections: (HashSet<i32>, HashSet<i32>) = parse_ranges(input);
        assert_eq!(
            (HashSet::from([2, 3, 4]), HashSet::from([6, 7, 8])),
            sections
        )
    }

    #[test]
    fn determine_if_one_tuple_component_completely_contains_the_other() {
        let given = (HashSet::from([2, 3, 4]), HashSet::from([6, 7, 8]));
        let actual = one_tuple_component_completely_contains_the_other(given);
        assert!(!actual);

        let given = (HashSet::from([2, 3, 4]), HashSet::from([3]));
        let actual = one_tuple_component_completely_contains_the_other(given);
        assert!(actual);
    }

    #[test]
    fn determine_if_tuple_components_overlap() {
        let given = (HashSet::from([2, 3, 4]), HashSet::from([6, 7, 8]));
        let actual = tuple_components_overlap(given);
        assert!(!actual);

        let given = (HashSet::from([2, 3, 4]), HashSet::from([4, 5]));
        let actual = tuple_components_overlap(given);
        assert!(actual);
    }
}

fn one_tuple_component_completely_contains_the_other<T>(tuple: (HashSet<T>, HashSet<T>)) -> bool
where
    T: Eq + Hash,
{
    let first = tuple.0;
    let second = tuple.1;

    first.is_superset(&second) || first.is_subset(&second)
}

fn tuple_components_overlap<T>(tuple: (HashSet<T>, HashSet<T>)) -> bool
where
    T: Eq + Hash,
{
    let first = tuple.0;
    let second = tuple.1;
    !first.is_disjoint(&second)
}

fn parse_ranges(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let (part_one, part_two) = line.split_once(',').unwrap();
    let set_one = set_for_single_range(part_one);
    let set_two = set_for_single_range(part_two);
    (set_one, set_two)
}

fn set_for_single_range(string: &str) -> HashSet<i32> {
    let values = string.split_once('-').unwrap();
    let start: i32 = values.0.parse().unwrap();
    let end: i32 = values.1.parse().unwrap();
    let mut set = HashSet::new();
    for i in start..=end {
        set.insert(i);
    }
    set
}
