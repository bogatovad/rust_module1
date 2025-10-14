use std::fs;
use mt940::parse_mt940;
use mt940::Message;

fn read_mt940_file(file_path: &str) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let messages: Vec<Message> = parse_mt940(&content)?;
    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_mt940_test() {
        //act
        let file_path = "statement.mt940";

        //arrange
        let result: Vec<Message> = read_mt940_file(&file_path).unwrap();

        //assert
        for msg in result {
            println!("Transaction ref: {}", msg.transaction_ref_no);
            println!("Account ID: {}", msg.account_id);
            println!("Opening balance: {:?}", msg.opening_balance);
            println!("Closing balance: {:?}", msg.closing_balance);

            for line in msg.statement_lines.iter() {
                println!("  Line: {:?}", line);
            }
            println!("---");
        }
    }
}
