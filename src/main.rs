use std::fs::File;
use std::io::{BufRead, Cursor};
use std::io::prelude::*;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut savefile = File::open("/home/donal/projects/mocd/saves/mp_autosave.eu4")
        .expect("file opened");
    let mut savefile_string = String::new();
    
    savefile.read_to_string(&mut savefile_string).expect("savefile stringified");

    println!("Hello World!");
}
