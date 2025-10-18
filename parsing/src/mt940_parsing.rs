use mt940::parse_mt940;
use mt940::Message;


struct Transaction;

impl Transaction {
    fn read<R: std::io::Read>(input_reader: &mut R) -> Result<Vec<Message>, Box<dyn std::error::Error>>{
        let mut buf = Vec::new();
        input_reader.read_to_end(&mut buf);
        let content = String::from_utf8(buf).unwrap();
        let messages = parse_mt940(&content)?;
        Ok(messages)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_mt940_test() {
        //act
        let file_path = "statement.mt940";
        let f = std::fs::File::open(file_path);
        let mut file = match f {
            Ok(file) => file,
            _ => panic!("test panic.")
        };

        //arrange
        let result = Transaction::read(&mut file);
        let messages = match result {
            Ok(messages) => messages,
            _ => panic!("error")
        };
        //assert
        for msg in messages {
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

    #[test]
    fn parsing_mt940_test_stdint() {
        //act
        let data = ":20:123456789\n:25:123456789/12345678\n:28C:00001/001\n:60F:C240930EUR12345,67\n:61:2410011001D123,45NTRFNONREF//123456789\n:86:Transfer to John Doe\n:61:2410021002C456,78NTRFNONREF//987654321\n:86:Payment from ACME Corp\n:62F:C241002EUR12679,00\n".as_bytes();
        let mut cursor = std::io::Cursor::new(data);

        //arrange
        let result = Transaction::read(&mut cursor);
        let messages = match result {
            Ok(messages) => messages,
            _ => panic!("error")
        };
        //assert
        for msg in messages {
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
