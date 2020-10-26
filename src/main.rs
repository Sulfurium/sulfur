use lib::cli::cli::Cli;
use structopt::StructOpt;

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    if users::get_current_uid() == 0 {
        let opt = Cli::from_args();
        println!("{:#?}", opt);
    } else {
        print!("You not are in root ! Please rerun this command with root account or sudo");
    }

    Ok(())
}
