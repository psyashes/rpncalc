use clap::{Arg, App};

fn main() {
    let matches = App::new("rpncalc")
                        .version("1.0.0")
                        .author("psyashes")
                        .about("Calculater for Reverse Polish Notation")
                        .arg(Arg::with_name("formula_file")
                                .value_name("FILE")
                                .index(1)
                                .required(false))
                        .arg(Arg::with_name("verbose")
                                .short("v")
                                .long("verbose")
                                .required(false))
                        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified : {}", file),
        None => println!("No file specified!")
    }
    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
