use crate::alphabet::Alphabet;
use crate::math;

pub fn encrypt(x: &[char], k: (i32, i32)) -> Vec<char> {
    let alphabet = Alphabet;
    let (a, b) = k;
    x.iter()
        .map(|c| {
            let v = alphabet.index(*c) as i32 * a + b;
            let i = math::reminder(v, alphabet.len() as i32);
            alphabet.value(i as usize)
        })
        .collect()
}

pub fn decrypt(y: &[char], k: (i32, i32)) -> Vec<char> {
    let alphabet = Alphabet;
    let (a, b) = k;
    y.iter()
        .map(|c| {
            let v = (alphabet.index(*c) as i32 - b) * a;
            let i = math::reminder(v, alphabet.len() as i32);
            alphabet.value(i as usize)
        })
        .collect()
}

#[test]
fn t_encrypt() {
    let x = "ATTACK";
    let x: Vec<char> = x.chars().collect();
    let y = "NCCNFZ";
    let y: Vec<char> = y.chars().collect();
    assert_eq!(encrypt(&x, (9, 13)), y);
}

#[test]
fn t_decrypt() {
    let x = "ATTACK";
    let x: Vec<char> = x.chars().collect();
    let y = "NCCNFZ";
    let y: Vec<char> = y.chars().collect();
    assert_eq!(decrypt(&y, (3, 13)), x);
}
