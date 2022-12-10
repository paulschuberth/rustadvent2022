use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use nom::combinator::value;

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::day6::{start_of_transmission_packet, Buffer};

    #[test]
    fn can_add_elements_to_buffer() {
        let mut buffer: Buffer<usize> = Buffer::from(vec![]);
        buffer.add(1);
        assert_eq!(buffer.0, VecDeque::from([1]));
        assert_eq!(1, buffer.len());
    }

    #[test]
    fn can_remove_elements_from_buffer() {
        let mut buffer: Buffer<usize> = Buffer::from(vec![1, 2, 3]);
        let e_option: Option<usize> = buffer.remove();
        assert!(e_option.is_some());
        assert_eq!(1, e_option.unwrap());
        assert_eq!(2, buffer.len());
    }

    #[test]
    fn buffer_can_contain_four_elements() {
        let mut buffer = Buffer::from(vec!['a', 'b', 'c']);
        buffer.add('d');
        assert_eq!(4, buffer.len());
        buffer.add('e');
        assert_eq!(4, buffer.len());
    }

    #[test]
    fn can_determine_if_all_contained_items_are_distinct() {
        let buffer = Buffer::from(vec![1, 1]);
        assert!(!buffer.are_items_distinct());

        let buffer = Buffer::from(vec![1, 2]);
        assert!(buffer.are_items_distinct());
    }

    #[test]
    fn can_determine_if_buffer_it_at_capacity() {
        let buffer = Buffer::from(vec![1, 2, 3]);
        assert!(!buffer.is_at_capacity());

        let buffer = Buffer::from(vec![1, 2, 3, 4]);
        assert!(buffer.is_at_capacity());
    }

    #[test]
    fn determines_start_of_packet() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let index = start_of_transmission_packet(input);
        assert_eq!(5, index.unwrap());

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let index = start_of_transmission_packet(input);
        assert_eq!(6, index.unwrap());

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let index = start_of_transmission_packet(input);
        assert_eq!(10, index.unwrap());

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let index = start_of_transmission_packet(input);
        assert_eq!(11, index.unwrap());
    }
}

pub(crate) fn solve() {
    println!("--------------------------------------");
    println!("Day 6");
    println!("--------------------------------------");
    let input = std::fs::read_to_string("input/day6.txt").expect("Input file is missing");

    let index = start_of_transmission_packet(input.as_str());
    println!("Day 6 Part 1: {}", index.unwrap());
}

fn start_of_transmission_packet(input: &str) -> Option<usize> {
    let mut buffer: Buffer<char> = Buffer::from(vec![]);
    for (i, c) in input.chars().enumerate() {
        buffer.add(c);
        if buffer.is_at_capacity() && buffer.are_items_distinct() {
            return Some(i + 1);
        }
    }
    None
}

struct Buffer<T>(VecDeque<T>);

impl<T> Buffer<T>
where
    T: Eq + Hash,
{
    fn is_at_capacity(&self) -> bool {
        self.0.len() == 4
    }

    fn are_items_distinct(&self) -> bool {
        self.0.len() == self.0.iter().count_unique()
    }

    fn from(vec: Vec<T>) -> Self {
        let deque = VecDeque::from(vec);
        Buffer(deque)
    }

    fn add(&mut self, e: T) {
        self.0.push_back(e);
        if self.len() > 4 {
            self.0.pop_front();
        }
    }

    fn remove(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

trait CountUnique {
    fn count_unique(self) -> usize;
}

impl<I, T> CountUnique for I
where
    I: Iterator<Item = T>,
    T: Eq + Hash,
{
    fn count_unique(self) -> usize {
        self.collect::<HashSet<_>>().len()
    }
}
