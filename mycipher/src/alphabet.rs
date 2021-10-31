use std::collections::HashMap;

pub type FreqChar = (char, f32);

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

impl From<&[FreqChar]> for Alphabet {
    fn from(chars: &[FreqChar]) -> Self {
        // sort chars Hi freq -> Low freq
        let mut chars = chars.to_vec();
        chars.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut alphabet = Vec::with_capacity(chars.len());
        for it in chars.iter().rev() {
            alphabet.push(it.0);
        }
        Self { alphabet }
    }
}

// build alphabet with frequency
impl From<&str> for Alphabet {
    fn from(content: &str) -> Self {
        let mut freq = HashMap::new();

        // build frequency table
        for c in content.chars() {
            if c.is_alphabetic() {
                let counter = freq.entry(c).or_insert(0.0);
                *counter += 1.0;
            }
        }

        let freq_chars: Vec<FreqChar> = freq.iter().map(|(&c, &f)| (c, f)).collect();
        Alphabet::from(freq_chars.as_slice())
    }
}

#[test]
fn t_alphabet_frequency() {
    let mut content = String::new();
    let mut count = 1;
    for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        // repeat char count times
        let chars = String::from(c).repeat(count);
        content.push_str(&chars);
        count += 1;
    }
    let revers_standard: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().rev().collect();
    let alphabet = Alphabet::from(content.as_str());
    assert_eq!(revers_standard, alphabet.alphabet)
}

#[test]
fn t_alphabet_char_freq() {
    let chars_freq = vec![('A', 1.0), ('B', 2.0), ('C', 3.0), ('D', 4.0), ('E', 5.0)];
    let alphabet = Alphabet::from(chars_freq.as_slice());
    assert_eq!(vec!['E', 'D', 'C', 'B', 'A'], alphabet.alphabet);
}
