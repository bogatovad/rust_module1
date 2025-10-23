use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
   #[arg(long)]
   pub input: String,

   #[arg(long)]
   pub in_format: String,

   #[arg(long)]
   pub output: String,

   #[arg(long)]
   pub out_format: String
}

impl Cli {
    pub fn parse_arg() -> Cli{
        Cli::parse()
    }
}