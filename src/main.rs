use clap::Clap;

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
}
