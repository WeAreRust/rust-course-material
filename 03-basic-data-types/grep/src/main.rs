use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::str;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let pattern = &args[1];
    let path = &args[2];

    grep(Path::new(&path), pattern)?;
    Ok(())
}

pub fn grep(dir: &Path, pattern: &str) -> Result<(), io::Error> {
    if !dir.is_dir() {
        return Ok(());
    }

    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            grep(&path, pattern)?;
        } else {
            match path.file_name() {
                Some(filename) => {
                    match_contents(&path, pattern, filename.to_str().expect("invalid filename encoding"))?;
                }
                None => continue
            }
        }
    }

    Ok(())
}

fn match_contents(path: &Path, pattern: &str, filename: &str) -> Result<(), io::Error> {
    // skip non-utf8 encoded files
    if let Ok(contents) = fs::read_to_string(&path) {
        if let Some(index) = contents.find(pattern) {
            print_match(filename, &contents.as_bytes(), pattern, index);
        }
    }

    Ok(())
}

const NEWLINE_BYTE: u8 = 0xA;

fn print_match(filename: &str, contents: &[u8], _pattern: &str, index: usize) {
    let mut line_start = 0;
    let mut current = index;
    while current > 0 {
        current -= 1;
        if contents[current] == NEWLINE_BYTE {
            line_start = current;
            break;
        }
    }

    let mut line_end = contents.len();
    current = index;
    while current < contents.len() {
        current += 1;
        if contents[current] == NEWLINE_BYTE {
            line_end = current;
            break;
        }
    }

    if let Ok(line) = String::from_utf8(contents[line_start..line_end].to_vec()) {
        println!("{}:{}", filename, line);
    }
}
