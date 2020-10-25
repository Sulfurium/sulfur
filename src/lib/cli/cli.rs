use clap::{App, Arg, SubCommand};

pub fn cli() {
    let matches = App::new("Sulfurium OS package Manager")
        .version("0.1.0")
        .author("Rheydskey")
        .about("This is the package manager of Sulfurium Os fast and Rusted Distro")
        //.arg(Arg::with_name("Action").required(true))
        .arg(Arg::with_name("install")
            .short("i")
            .long("install")
            .value_name("Package")
            .help("Install a package")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("Action").unwrap());

    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

}