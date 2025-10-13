mod args;
use crate::args::Cli;


fn main() {
    let cli = Cli::parse_arg();
    println!("Name: {:?}", cli.input);
    println!("Name: {:?}", cli.in_format);
}