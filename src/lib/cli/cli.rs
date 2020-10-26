use structopt::StructOpt;

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
}
