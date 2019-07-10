use std::env;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    for arg in env::args().skip(1) {
        io::copy(&mut File::open(arg)?, &mut io::stdout())?;
    }

    Ok(())
}
