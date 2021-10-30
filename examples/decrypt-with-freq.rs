use mycipher::alphabet::Alphabet;
use mycipher::util;
use std::env;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let f_model = args.next().expect("invalid arg");
    let f_encrypt = args.next().expect("invalid arg");

    let content_model = util::read_contents(f_model.as_str()).expect("invalid file content");
    let content_encrypt = util::read_contents(f_encrypt.as_str()).expect("invalid file content");

    let model_alphabet = Alphabet::from(content_model.as_str());
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

    println!("{}", decrypt);
}
