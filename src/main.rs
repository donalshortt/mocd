use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let mut savefile = File::open("/home/donal/projects/mocd/saves/mp_autosave.eu4")
        .expect("file opened");



    println!("Hello World!");
}
