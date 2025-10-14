use serde::{Serialize, Deserialize};
use chrono::NaiveDate;


#[derive(Serialize, Deserialize, Debug)]
struct Operation {
    date: NaiveDate,
    amount: f32,
    currency: String,
    description: String,
    reference: String
}

fn read_csv_file(file_path: &str) -> Result<Vec<Operation>, Box<dyn std::error::Error>>{
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut csv_parsed_data: Vec<Operation> = Vec::new();

    // read file line by line.
    for result in reader.deserialize() {
        let op: Operation = result?;
        csv_parsed_data.push(op);
    }

    Ok(csv_parsed_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_csv_test() {
        //act
        let file_path = "in_data.csv";

        //arrange
        let result: Vec<Operation> = read_csv_file(&file_path).unwrap();

        //assert
        assert_eq!(result[0].amount, -1000.00);
        assert_eq!(result[0].currency, "EUR");
        assert_eq!(result[0].description, "Payment to supplier");
        assert_eq!(result[0].reference, "REF123456");

        assert_eq!(result[1].amount, 2500.00);
        assert_eq!(result[1].currency, "EUR");
        assert_eq!(result[1].description, "Client payment");
        assert_eq!(result[1].reference, "REF789012");
    }
}
