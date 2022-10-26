use std::fs::File;
use std::io::{BufRead, Cursor};
use std::io::prelude::*;

fn main() {
    let mut savefile = File::open("../saves/mp_autosave.eu4")
        .expect("file opened");
    let mut savefile_string = String::new();
    
    savefile.read_to_string(&mut savefile_string)
        .expect("savefile stringified");
}
