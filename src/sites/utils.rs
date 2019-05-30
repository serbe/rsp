use std::fs::File;
use std::io::Write;

pub fn rot13(text: &str) -> String {
    text.chars().map(|c| {
        match c {
            'A' ... 'M' | 'a' ... 'm' => ((c as u8) + 13) as char,
            'N' ... 'Z' | 'n' ... 'z' => ((c as u8) - 13) as char,
            _ => c
        }
    }).collect()
}

//pub fn rotate(src: Vec<String>, num: usize) -> Vec<String> {
//	let mut s = &src;
//	s.rotate_left(num);
//	s.to_vec()
//}

pub fn save(name: &str, body: &str) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    file.write_all(body.as_bytes())
}