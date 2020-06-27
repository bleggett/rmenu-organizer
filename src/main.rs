use std::fs::File;
use std::io::Error;

fn main() {
    read_disc_ccd("./Battle Garegga (Japan)/Battle Garegga (Japan).img");
}

fn read_disc_ccd(img_name: &str) -> Result<(), Error> {
    let mut file = match File::open(img_name) {
        Err(e) => {
            println!("Error opening image file, error was {0}", e);
            return Err(e);
        },
        Ok(f) => return Ok(()),
    };
}
