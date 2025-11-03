mod args;

use crate::args::Cli;
use parsing::cam_struct::Document;
use parsing::mt940_parsing::Mt940Wrapper;
use parsing::csv_parsing::Transaction;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse_arg();

    match (cli.in_format.as_str(), cli.out_format.as_str()) {
        ("camt053", "mt940") => {
            let mut file_in = std::fs::File::open(&cli.input)?;
            let mut file_out = std::fs::File::create(&cli.output)?;
            let doc_camt053 = Document::read(&mut file_in)?;
            let doc_mt940 = doc_camt053.to_mt940()?;
            doc_mt940.write(&mut file_out)?;
        }
        ("mt940", "camt053") => {
            let mut file_in = std::fs::File::open(&cli.input)?;
            let mut file_out = std::fs::File::create(&cli.output)?;
            let doc_mt940 = Mt940Wrapper::read(&mut file_in)?;
            let mut doc_camt053 = doc_mt940.to_camt053()?;
            doc_camt053.write(&mut file_out)?;
        }
        ("mt940", "stdout") => {
            let mut file_in = std::fs::File::open(&cli.input)?;
            let doc_mt940 = Mt940Wrapper::read(&mut file_in)?;
            println!("{}", doc_mt940.to_mt_string());
        }
        ("camt053", "stdout") => {
            let mut file_in = std::fs::File::open(&cli.input)?;
            let mut doc_camt053 = Document::read(&mut file_in)?;
            println!("{:?}", doc_camt053.to_string());
        }
        ("csv", "stdout") => {
            let mut file_in = std::fs::File::open(&cli.input)?;
            let csv_data = Transaction::read(&mut file_in)?;
            println!("{:?}", csv_data);
        }
        ("stdout", "csv") => {
            let mut file_out = std::fs::File::create(&cli.output)?;
            let data_in = cli.input.as_bytes();
            let mut reader = std::io::Cursor::new(data_in);
            let mut data = Transaction::read(&mut reader)?;
            data.write(&mut file_out)?;
        }
        ("stdout", "mt940") => {
            let mut file_out = std::fs::File::create(&cli.output)?;
            let data_in = cli.input.as_bytes();
            let mut reader = std::io::Cursor::new(data_in);
            let data = Mt940Wrapper::read(&mut reader)?;
            data.write(&mut file_out)?;
        }
        ("stdout", "camt053") => {
            let mut file_out = std::fs::File::create(&cli.output)?;
            let data_in = cli.input.as_bytes();
            let mut reader = std::io::Cursor::new(data_in);
            let mut data = Document::read(&mut reader)?;
            data.write(&mut file_out)?;
        }
        _ => eprintln!("Unsupported format combination: {} → {}", cli.in_format, cli.out_format),
    }

    Ok(())
}