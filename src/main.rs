use crate::lib::default::default::default;
use crate::lib::packages::install::install;
use lib::{
    cli::cli::Cli,
    packages::{appimage::get::get_appimage, query::query, remove::remove},
};
use structopt::StructOpt;
mod lib;

#[tokio::main]
async fn main() {
    if users::get_current_uid() == 0 {
        default().await.expect("Error");

        match Cli::from_args() {
            Cli::Install { packages } => install(packages).await,
            Cli::Query { packages } => query(packages).await,
            Cli::Remove { packages } => remove(packages).await,
        };
    } else {
        print!("You not are in root ! Please rerun this command with root account or sudo");
    }
}
