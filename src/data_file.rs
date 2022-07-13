use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use clap::StructOpt;

pub(crate) fn load_kask(filepath_string :String){
    let filepath = Path::new(&filepath_string);
    println!("{}",filepath.display());
    let mut file =  File::open(filepath).expect("Error while reading File");
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    println!("{}",contents)


}