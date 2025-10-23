mod args;

use crate::args::Cli;
use parsing::cam_struct::Document;
use parsing::mt940_parsing::Mt940Wrapper;
use parsing::csv_parsing::Transaction;



fn main(){
    let cli = Cli::parse_arg();
    if cli.in_format == "camt053" && cli.out_format == "mt940"{
        let mut file_in_format = std::fs::File::open(&cli.input).unwrap();
        let mut file_out_format = std::fs::File::create(&cli.output).unwrap();
        let doc_cam053 = Document::read(&mut file_in_format).unwrap();
        let doc_mt940 = doc_cam053.to_mt940().unwrap();
        let _ = doc_mt940.write(&mut file_out_format);
    }
    if cli.in_format == "mt940" && cli.out_format == "camt053"{
        let mut file_in_format = std::fs::File::open(&cli.input).unwrap();
        let mut file_out_format = std::fs::File::create(&cli.output).unwrap();
        let doc_mt940 = Mt940Wrapper::read(&mut file_in_format).unwrap();
        let mut doc_cam053 = doc_mt940.to_camt053().unwrap();
        let _ = doc_cam053.write(&mut file_out_format);
    }
    if cli.in_format == "mt940" && cli.out_format == "stdout"{
        let mut file_in_format = std::fs::File::open(&cli.input).unwrap();
        let doc_mt940 = Mt940Wrapper::read(&mut file_in_format).unwrap();
        println!("{:?}", doc_mt940.to_mt_string());
    }

    if cli.in_format == "camt053" && cli.out_format == "stdout"{
        let mut file_in_format = std::fs::File::open(&cli.input).unwrap();
        let mut doc_camt053 = Document::read(&mut file_in_format).unwrap();
        println!("{:?}", doc_camt053.to_string());
    }
    if cli.in_format == "csv" && cli.out_format == "stdout"{
        let mut file_in_format = std::fs::File::open(&cli.input).unwrap();
        let csv_data = Transaction::read(&mut file_in_format).unwrap();
    }
    if cli.in_format == "stdout" && cli.out_format == "csv"{
        let mut file_out_format = std::fs::File::create(&cli.output).unwrap();
        let mut data_in = cli.input.as_bytes();
        let mut reader = std::io::Cursor::new(data_in);
        let mut data = Transaction::read(&mut reader).unwrap();
        let _ = data.write(&mut file_out_format);
    }
    if cli.in_format == "stdout" && cli.out_format == "mt940"{
        let mut file_out_format = std::fs::File::create(&cli.output).unwrap();
        let mut data_in = cli.input.as_bytes();
        let mut reader = std::io::Cursor::new(data_in);
        let mut data = Mt940Wrapper::read(&mut reader).unwrap();
        let _ = data.write(&mut file_out_format);
    }
    if cli.in_format == "stdout" && cli.out_format == "camt053"{
        let mut file_out_format = std::fs::File::create(&cli.output).unwrap();
        let mut data_in = cli.input.as_bytes();
        let mut reader = std::io::Cursor::new(data_in);
        let mut data = Document::read(&mut reader).unwrap();
        let _ = data.write(&mut file_out_format);
    }
}