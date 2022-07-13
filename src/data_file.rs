use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use clap::StructOpt;

fn load_kask(filepath_string :String){
    let filepath = Path::new(&filepath_string);
    let mut file =  File::open(filepath);
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!(contents)


}