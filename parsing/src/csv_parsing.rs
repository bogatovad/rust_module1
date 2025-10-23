use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Line {
    date: NaiveDate,
    amount: f32,
    currency: String,
    description: String,
    reference: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction  {
   lines: Vec<Line>
}


impl Transaction  {
    pub fn read<R: std::io::Read>(input_reader: &mut R) -> Result<Transaction, Box<dyn std::error::Error>>{
        // create reader.
        let mut reader = csv::ReaderBuilder::new().from_reader(input_reader);
        let mut lines: Vec<Line> = Vec::new();

        
        // read file line by line.
        for result in reader.deserialize() {
            let op: Line  = result?;
            lines.push(op);
        }

        Ok(Transaction {lines: lines})
    }

    pub fn write<W: std::io::Write>(&mut self, input_writer: &mut W)-> Result<(), Box<dyn std::error::Error>>{
        // create writer.
        let mut writer = csv::WriterBuilder::new().from_writer(input_writer);
        
        for line in &self.lines {
            let _ = writer.serialize(&line);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_csv_test_stdin() {
        //arrange
        let data = "date,amount,currency,description,reference\n2023-10-01,-1000.00,EUR,Payment to supplier,REF123456\n2023-10-02,2500.00,EUR,Client payment,REF789012".as_bytes();
        let mut cursor = std::io::Cursor::new(data);

        //act
        let result = Transaction::read(&mut cursor);
        
        let result = match result {
            Ok(res) => res,
            _ => panic!("fd")
            
        };

        //assert
        assert_eq!(result.lines[0].amount, -1000.00);
        assert_eq!(result.lines[0].currency, "EUR");
        assert_eq!(result.lines[0].description, "Payment to supplier");
        assert_eq!(result.lines[0].reference, "REF123456");

        assert_eq!(result.lines[1].amount, 2500.00);
        assert_eq!(result.lines[1].currency, "EUR");
        assert_eq!(result.lines[1].description, "Client payment");
        assert_eq!(result.lines[1].reference, "REF789012");
    }

    #[test]
    fn parsing_csv_test_file() {
        //arrange
        let file_path = "in_data.csv";
        let r = &mut std::fs::File::open(file_path);
        
        let file = match r {
            Ok(file) => file,
            _ => panic!("test panic.")
        };

        //act
        let result = Transaction::read(file);
        
        let result = match result {
            Ok(res) => res,
            _ => panic!("fd")
            
        };

        //assert
        assert_eq!(result.lines[0].amount, -1000.00);
        assert_eq!(result.lines[0].currency, "EUR");
        assert_eq!(result.lines[0].description, "Payment to supplier");
        assert_eq!(result.lines[0].reference, "REF123456");

        assert_eq!(result.lines[1].amount, 2500.00);
        assert_eq!(result.lines[1].currency, "EUR");
        assert_eq!(result.lines[1].description, "Client payment");
        assert_eq!(result.lines[1].reference, "REF789012");
    }

    #[test]
    fn write_test_to_file(){

        //arrange
        let file_path = "in_data_write_test.csv";
        let r = &mut std::fs::File::create(file_path);
        let lines = Vec::from([Line{
            date: NaiveDate::from_ymd_opt(2023, 12, 31).expect("Valid date"),
            amount: 100.5,
            currency: String::from("RUB"),
            description: String::from("Test write csv."),
            reference: String::from("Some test ref"),
        }]);
        let mut data = Transaction{ lines: lines };
        let file = match r {
            Ok(file) => file,
            _ => panic!("test panic.")
        };

        //act
        let _ = data.write(file);
        
        //assert
        let data_from_file= std::fs::read_to_string(file_path);
        assert_eq!(data_from_file.unwrap(), "date,amount,currency,description,reference\n2023-12-31,100.5,RUB,Test write csv.,Some test ref\n");
        let _ = std::fs::remove_file(file_path);
    }

    #[test]
    fn write_test_to_stdin(){

        //arrange
        let mut vec = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut vec);
        let lines = Vec::from([Line{
            date: NaiveDate::from_ymd_opt(2023, 12, 31).expect("Valid date"),
            amount: 100.5,
            currency: String::from("RUB"),
            description: String::from("Test write csv."),
            reference: String::from("Some test ref"),
        }]);
        let mut data = Transaction{ lines: lines };

        //act
        let _ = data.write( &mut cursor);

        //assert
        let result_string = String::from_utf8(vec).unwrap();
        println!("{}", result_string);
        assert_eq!(result_string, "date,amount,currency,description,reference\n2023-12-31,100.5,RUB,Test write csv.,Some test ref\n");
    }
}
