use structopt::StructOpt;
use std::str::FromStr;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Sulfur -",
    version = "0.0.1",
    author = "By Rheydskey and Sulfurium OS team",
    about = "Package Manager of Sulfurium OS"
)]
pub enum Cli {
    /// Install a package
    Install {
        #[structopt()]
        packages: Vec<String>,
    },
    /// Remove a package
    Remove {
        #[structopt()]
        packages: Vec<String>,
    },
    /// Search a package
    Query {
        #[structopt()]
        packages: Vec<String>,
    },
    /// Sync a repo
    Sync,
    /// Add / Remove / Edit repo
    Repo {
        #[structopt()]
        action: Action,

        #[structopt()]
        repo_name: String,

        #[structopt(default_value = "")]
        url: String
    }
}
#[derive(Debug, StructOpt)]
pub enum Action {
    Add,
    Del,
    Update,
    Error
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s.to_lowercase().as_str() {
           "add" => {
                Ok(Self::Add)
           }
           "del" => {
               Ok(Self::Del)
           }
           "update" => {
               Ok(Self::Update)
           }
           &_ => {
            Err(String::new())
           }
       }
    }
}
