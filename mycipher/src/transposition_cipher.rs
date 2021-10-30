const ALPHABET: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const M: i32 = 26;

// modulo op with negative number
fn reminder(a: i32) -> i32 {
    ((a % M) + M) % M
}

pub fn encrypt(x: &[char], k: i32) -> Vec<char> {
    x.iter()
        .map(|c| {
            let i = reminder((*c as i32 - 'A' as i32) + k);
            ALPHABET[i as usize]
        })
        .collect()
}

pub fn decrypt(x: &[char], k: i32) -> Vec<char> {
    x.iter()
        .map(|c| {
            let i = reminder((*c as i32 - 'A' as i32) - k);
            ALPHABET[i as usize]
        })
        .collect()
}

#[test]
fn t_encrypt() {
    let x = "ATTACK";
    let x: Vec<char> = x.chars().collect();
    let y = "RKKRTB";
    let y: Vec<char> = y.chars().collect();
    assert_eq!(encrypt(&x, 17), y);
}

#[test]
fn t_decrypt() {
    let x = "ATTACK";
    let x: Vec<char> = x.chars().collect();
    let y = "RKKRTB";
    let y: Vec<char> = y.chars().collect();
    assert_eq!(decrypt(&y, 17), x);
}

#[test]
fn t_reminder() {
    assert_eq!(reminder(-7), 19);
}
