use crate::lib::default::default::default;
use crate::lib::packages::install::install;
use lib::{
    cli::cli::Cli,
    packages::{query::query, remove::remove},
};
use structopt::StructOpt;
mod lib;
use async_std::task::{self, spawn};
use task::block_on;

fn main() {
    block_on(spawn(async {
        if users::get_current_uid() == 0 {
            default().await.expect("Error");
            match Cli::from_args() {
                Cli::Install { packages } => install(packages).await,
                Cli::Query { packages } => query(packages).await,
                Cli::Remove { packages } => remove(packages).await,
            };
        } else {
            println!("You not are in root ! Please rerun this command with root account or sudo");
        }
    }));
}
