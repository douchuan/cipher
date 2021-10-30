use crate::alphabet::Alphabet;
use crate::math;

pub fn encrypt(x: &[char], k: i32) -> Vec<char> {
    let alphabet = Alphabet::default();
    x.iter()
        .map(|c| {
            let v = alphabet.index(*c) as i32 + k;
            let i = math::reminder(v, alphabet.len() as i32);
            alphabet.value(i as usize)
        })
        .collect()
}

pub fn decrypt(y: &[char], k: i32) -> Vec<char> {
    let alphabet = Alphabet::default();
    y.iter()
        .map(|c| {
            let v = alphabet.index(*c) as i32 - k;
            let i = math::reminder(v, alphabet.len() as i32);
            alphabet.value(i as usize)
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
    assert_eq!(math::reminder(-7, 26), 19);
}
