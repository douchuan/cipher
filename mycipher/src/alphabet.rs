const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Alphabet;

impl Alphabet {
    pub fn index(&self, c: char) -> usize {
        c as usize - 'A' as usize
    }

    pub fn len(&self) -> usize {
        ALPHABET.len()
    }

    pub fn value(&self, i: usize) -> char {
        ALPHABET[i]
    }
}
