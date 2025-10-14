mod args;
use crate::args::Cli;

fn main(){
    let cli = Cli::parse_arg();
    println!("{}", cli.in_format);
    println!("{}", cli.input)
}