use crate::lib::default::default::default;
use crate::lib::packages::install::install;
use lib::{
    cli::cli::Cli,
    packages::{query::query, remove::remove},
};
use structopt::StructOpt;

mod lib;

#[tokio::main]
async fn main() {
    default().await.expect("Error");

    if users::get_current_uid() == 0 {
        match Cli::from_args() {
            Cli::Install { packages } => install(packages).await,
            Cli::Query { packages } => query(packages).await,
            Cli::Remove { packages } => remove(packages),
        };
    } else {
        print!("You not are in root ! Please rerun this command with root account or sudo");
    }
}
