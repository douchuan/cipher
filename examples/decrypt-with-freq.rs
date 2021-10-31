use mycipher::alphabet::Alphabet;
use mycipher::util;
use std::collections::HashSet;
use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next();

    let model_alphabet = create_alphabet();

    let f_encrypt = args.next().expect("invalid arg");
    let content_encrypt = util::read_contents(f_encrypt.as_str()).expect("invalid file content");
    let encrypt_alphabet = Alphabet::from(content_encrypt.as_str());

    let mut decrypt = String::new();
    for c in content_encrypt.chars() {
        let c = if c.is_alphabetic() {
            let i = encrypt_alphabet.index(c);
            model_alphabet.value(i)
        } else {
            c
        };
        decrypt.push(c);
    }

    println!("{}", decrypt.to_lowercase());
}

fn create_alphabet() -> Alphabet {
    // Understanding Cryptography: A Textbook for Students and Practitioners
    // Table 1-1
    let freq_chars = vec![
        ('A', 0.0817),
        ('B', 0.015),
        ('C', 0.0278),
        ('D', 0.0425),
        ('E', 0.127),
        ('F', 0.0223),
        ('G', 0.0202),
        ('H', 0.0609),
        ('I', 0.0697),
        ('J', 0.0015),
        ('K', 0.0077),
        ('L', 0.0403),
        ('M', 0.0241),
        ('N', 0.0675),
        ('O', 0.0751),
        ('P', 0.0193),
        ('Q', 0.001),
        ('R', 0.0599),
        ('S', 0.0633),
        ('T', 0.0906),
        ('U', 0.0276),
        ('V', 0.0098),
        ('W', 0.0236),
        ('X', 0.0015),
        ('Y', 0.0197),
        ('Z', 0.0007),
    ];
    assert_eq!(26, freq_chars.len());
    let set: HashSet<char> = freq_chars.iter().map(|v| v.0).collect();
    assert_eq!(26, set.len());
    Alphabet::from(freq_chars.as_slice())
}
