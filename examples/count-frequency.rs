use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::fmt::{Display, Formatter};
use std::io::Read;

const TOP: usize = 10;

#[derive(Debug, Default)]
struct Count<T> {
    sum: usize,
    tops: Vec<(T, usize, f32)>,
}

// (item, count)
struct CountItem<T>(T, usize);

fn main() {
    let mut args = env::args();
    if args.len() > 1 {
        if let Some(file) = args.next_back() {
            let content = read_contents(file.as_str()).unwrap();

            match count_letters(content.as_str()) {
                Ok(c) => println!("chars: {}", c),
                _ => (),
            }

            match count_words(content.as_str()) {
                Ok(c) => println!("words: {}", c),
                _ => (),
            }
        }
    }
}

fn count_letters(content: &str) -> std::io::Result<Count<char>> {
    // count of chars
    let mut count = Count::default();
    let mut char2count = HashMap::new();
    for c in content.chars() {
        if !c.is_whitespace() {
            count.sum += 1;
            let n = match char2count.get(&c) {
                Some(n) => n + 1,
                None => 0,
            };
            char2count.insert(c, n);
        }
    }

    find_top(&char2count, &mut count);

    Ok(count)
}

fn count_words(content: &str) -> std::io::Result<Count<&str>> {
    let mut count = Count::default();
    let mut word2count = HashMap::new();
    for word in content.split(' ') {
        if word.chars().all(|c| c.is_whitespace()) {
            continue;
        }

        count.sum += 1;
        let n = match word2count.get(word) {
            Some(n) => n + 1,
            None => 0,
        };
        word2count.insert(word, n);
    }

    find_top(&word2count, &mut count);

    Ok(count)
}

fn read_contents(file: &str) -> std::io::Result<String> {
    let mut file = std::fs::File::open(file)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn find_top<T>(map: &HashMap<T, usize>, count: &mut Count<T>)
where
    T: Clone,
{
    let mut heap = BinaryHeap::new();
    for (c, n) in map {
        heap.push(CountItem(c, *n));
    }

    for _ in 0..TOP {
        match heap.pop() {
            Some(it) => {
                count
                    .tops
                    .push((it.0.clone(), it.1, it.1 as f32 / count.sum as f32));
            }
            _ => break,
        }
    }
}

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

impl<T> std::fmt::Display for Count<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut content = String::new();
        content.push_str(&format!("Sum = {}, top-{}: \n", self.sum, self.tops.len()));
        for (v, _, f) in self.tops.iter() {
            content.push_str(&format!("  ({}, {:.2}%)\n", v, *f * 100.0));
        }
        write!(f, "{}", content)
    }
}
