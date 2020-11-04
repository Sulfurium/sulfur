use crate::lib::default::default::default;
use crate::lib::packages::install::install;
use lib::cli::cli::Cli;
use structopt::StructOpt;

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    default().await.expect("Error");

    if users::get_current_uid() == 0 {
        match Cli::from_args() {
            Cli::Install { packages } => install(packages).await,
            Cli::Query { packages } => println!("{:?}", packages),
            Cli::Remove { packages } => println!("{:?}", packages),
        };
    } else {
        print!("You not are in root ! Please rerun this command with root account or sudo");
    }

    Ok(())
}
