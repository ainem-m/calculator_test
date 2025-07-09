use clap::Parser;
use calculator_cli::calculate;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    value1: f64,

    #[clap(value_parser)]
    operator: String,

    #[clap(value_parser)]
    value2: f64,
}

fn main() {
    let args = Args::parse();

    match calculate(args.value1, &args.operator, args.value2) {
        Ok(result) => println!("{} {} {} = {}", args.value1, args.operator, args.value2, result),
        Err(e) => println!("{}", e),
    }
}
