pub struct Alphabet {
    alphabet: Vec<char>,
}

impl Alphabet {
    pub fn index(&self, c: char) -> usize {
        let c0 = self.alphabet[0];
        c as usize - c0 as usize
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
