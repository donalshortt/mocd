use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut savefile = File::open("/home/donal/projects/mocd/saves/mp_autosave.eu4")
        .expect("file opened");

    let mut buf = Vec::new();
    savefile.read_to_end(&mut buf)
        .expect("savefile vectored");

    let savefile_contents = String::from_utf8_lossy(&buf);

    let players_countries_index = savefile_contents.find("players_countries");



    println!("Hello World!");
}
