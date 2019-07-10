use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry in fs::read_dir(env::current_dir()?)? {
        let entry = entry?;
        println!("{}", entry.file_name().to_str().expect("invalid file name encoding"));
    }

    Ok(())
}
