use lib::cli::cli::cli;

mod lib;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    if users::get_current_uid() == 0 {
        cli()
    } else {
        print!("You not are in root ! Please rerun this command with root account or sudo");
    }

    Ok(())
}
