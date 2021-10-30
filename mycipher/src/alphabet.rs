use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub struct Alphabet {
    alphabet: Vec<char>,
}

impl Alphabet {
    pub fn index(&self, c: char) -> usize {
        self.alphabet.iter().position(|it| *it == c).unwrap()
    }

    pub fn len(&self) -> usize {
        self.alphabet.len()
    }

    pub fn value(&self, i: usize) -> char {
        self.alphabet[i]
    }
}

impl Default for Alphabet {
    fn default() -> Self {
        Self {
            alphabet: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        }
    }
}

impl From<Vec<char>> for Alphabet {
    fn from(alphabet: Vec<char>) -> Self {
        Self { alphabet }
    }
}

// build alphabet with frequency
impl From<&str> for Alphabet {
    fn from(content: &str) -> Self {
        let content: Vec<char> = content.chars().map(|c| c.to_ascii_lowercase()).collect();

        let mut char2count = HashMap::new();

        // count char
        for c in content {
            if c.is_alphabetic() {
                let n = match char2count.get(&c) {
                    Some(n) => n + 1,
                    None => 0,
                };
                char2count.insert(c, n);
            }
        }

        // top 26
        let mut heap = BinaryHeap::new();
        for (c, n) in char2count {
            heap.push(CountItem(c, n));
        }

        let mut alphabet = Vec::with_capacity(26);
        for _ in 0..26 {
            match heap.pop() {
                Some(it) => {
                    alphabet.push(it.0);
                }
                _ => break,
            }
        }

        Self { alphabet }
    }
}

// (item, count)
struct CountItem<T>(T, usize);
impl<T> Eq for CountItem<T> {}

impl<T> PartialEq<Self> for CountItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<T> PartialOrd<Self> for CountItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

impl<T> Ord for CountItem<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}
