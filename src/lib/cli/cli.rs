
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Sulfur -", version = "0.0.1", author = "By Rheydskey and Sulfurium OS team", about = "Package Manager of Sulfurium OS")]
pub enum Cli {
    /// Install a package
    Install {
        #[structopt()]
        package_name: Vec<String>,
    },
    /// Remove a package
    Remove {
        #[structopt()]
        package_name: Vec<String>,
    },
    /// Search a package
    Query {
        #[structopt()]
        package_name: Vec<String>,
    },
}