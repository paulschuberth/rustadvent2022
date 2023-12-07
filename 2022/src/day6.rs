use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[cfg(test)]
mod tests {
    use std::char;
    use std::collections::VecDeque;

    use crate::day6::Buffer;

    #[test]
    fn can_add_elements_to_buffer() {
        let mut buffer: Buffer<usize> = Buffer::with_limit(5);
        buffer.add(1);
        assert_eq!(buffer.deque, VecDeque::from([1]));
        assert_eq!(1, buffer.len());
    }

    #[test]
    fn buffer_can_contain_capacity_elements() {
        let mut buffer = Buffer::from(vec!['a', 'b', 'c'], 4);
        buffer.add('d');
        assert_eq!(4, buffer.len());
        buffer.add('e');
        assert_eq!(4, buffer.len());

        let mut buffer = Buffer::from(vec!['a'], 2);
        buffer.add('d');
        assert_eq!(2, buffer.len());
        buffer.add('e');
        assert_eq!(2, buffer.len());
    }

    #[test]
    fn can_determine_if_all_contained_items_are_distinct() {
        let buffer = Buffer::from(vec![1, 1], 4);
        assert!(!buffer.are_items_distinct());

        let buffer = Buffer::from(vec![1, 2], 4);
        assert!(buffer.are_items_distinct());
    }

    #[test]
    fn can_determine_if_buffer_it_at_capacity() {
        let buffer = Buffer::from(vec![1, 2, 3], 4);
        assert!(!buffer.is_at_capacity());

        let buffer = Buffer::from(vec![1, 2, 3, 4], 4);
        assert!(buffer.is_at_capacity());
    }

    #[test]
    fn determines_start_of_packet() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let mut buffer = Buffer::with_limit(4);
        let index = buffer.index_of_market(input);
        assert_eq!(5, index.unwrap());

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let mut buffer: Buffer<char> = Buffer::with_limit(4);
        let index = buffer.index_of_market(input);
        assert_eq!(6, index.unwrap());

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let mut buffer: Buffer<char> = Buffer::with_limit(4);
        let index = buffer.index_of_market(input);
        assert_eq!(10, index.unwrap());

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let mut buffer: Buffer<char> = Buffer::with_limit(4);
        let index = buffer.index_of_market(input);
        assert_eq!(11, index.unwrap());
    }

    #[test]
    fn determines_start_of_message() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let mut buffer: Buffer<char> = Buffer::with_limit(14);
        let index = buffer.index_of_market(input);
        assert_eq!(19, index.unwrap());

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let mut buffer: Buffer<char> = Buffer::with_limit(14);
        let index = buffer.index_of_market(input);
        assert_eq!(23, index.unwrap());

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let mut buffer: Buffer<char> = Buffer::with_limit(14);
        let index = buffer.index_of_market(input);
        assert_eq!(23, index.unwrap());

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let mut buffer: Buffer<char> = Buffer::with_limit(14);
        let index = buffer.index_of_market(input);
        assert_eq!(29, index.unwrap());

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let mut buffer: Buffer<char> = Buffer::with_limit(14);
        let index = buffer.index_of_market(input);
        assert_eq!(26, index.unwrap());
    }

    impl<T> Buffer<T> {
        fn from(vec: Vec<T>, capacity: usize) -> Self {
            Buffer {
                deque: VecDeque::from(vec),
                capacity,
            }
        }
    }
}

pub(crate) fn solve() {
    println!("--------------------------------------");
    println!("Day 6");
    println!("--------------------------------------");
    let input = std::fs::read_to_string("../input/day6.txt").expect("Input file is missing");

    let mut transmission_buffer: Buffer<char> = Buffer::with_limit(4);
    let index = transmission_buffer.index_of_market(input.as_str());
    println!("Day 6 Part 1: {}", index.unwrap());

    let mut transmission_buffer: Buffer<char> = Buffer::with_limit(14);
    let index = transmission_buffer.index_of_market(input.as_str());
    println!("Day 6 Part 2: {}", index.unwrap());
}

#[derive(Debug)]
struct Buffer<T> {
    deque: VecDeque<T>,
    capacity: usize,
}

impl<T> Buffer<T>
where
    T: Eq + Hash,
{
    fn is_at_capacity(&self) -> bool {
        self.deque.len() == self.capacity
    }

    fn are_items_distinct(&self) -> bool {
        self.deque.len() == self.deque.iter().count_unique()
    }

    fn with_limit(capacity: usize) -> Self {
        Buffer {
            deque: VecDeque::from(vec![]),
            capacity,
        }
    }

    fn add(&mut self, e: T) {
        self.deque.push_back(e);
        if self.len() > self.capacity {
            self.deque.pop_front();
        }
    }

    fn len(&self) -> usize {
        self.deque.len()
    }
}

impl Buffer<char> {
    fn index_of_market(&mut self, input: &str) -> Option<usize> {
        for (i, c) in input.chars().enumerate() {
            self.add(c);
            if self.is_at_capacity() && self.are_items_distinct() {
                return Some(i + 1);
            }
        }
        None
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
