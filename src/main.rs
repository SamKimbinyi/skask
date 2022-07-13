mod data_file;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author,version,about,long_about = None)]
struct Cli{
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands{
    /// Open a new or existing Skask datastore
    Open{
        #[clap(value_parser)]
        directory: Option<String>,

    },
}


fn main() {
    let cli = Cli::parse();

    match &cli.command{
            Commands::Open {directory} =>{
                println!("Opening {:?} ",directory);
            }
    }

}
