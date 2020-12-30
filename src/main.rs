use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clap, Debug)]
#[clap(
    name = "Reverse Polish Notation Calculater",
    version = "1.0.0",
    author = "psyashes",
    about = "Reverse Polish Notation Calculater"
)]

struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run (reader, opts.verbose);
    } else {
        println!("No file is specified");
    }
}

fn run(reader: BufReader<File>, _verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
