use std::io::Read;

pub fn read_contents(file: &str) -> std::io::Result<String> {
    let mut file = std::fs::File::open(file)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
