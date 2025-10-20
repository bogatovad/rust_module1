use swift_mt_message::messages::mt940::MT940;
use swift_mt_message::fields::field20::Field20;

pub struct Mt940Wrapper(pub MT940);

impl Mt940Wrapper {
    pub fn read<R: std::io::Read>(input_reader: &mut R) -> Result<Self, Box<dyn std::error::Error>>{
        let mut buf = Vec::new();
        let _ = input_reader.read_to_end(&mut buf);
        let content = String::from_utf8(buf).unwrap();
        Ok(Mt940Wrapper(MT940::parse_from_block4(&content)?))
    }

    pub fn write<W: std::io::Write>(&self, input_writer: &mut W) -> Result<(), Box<dyn std::error::Error>>{
        let mt_string = self.to_mt_string();
        let _ = input_writer.write_all(mt_string.as_bytes());
        Ok(())
    }

    pub fn to_camt053() -> Result<(), Box<dyn std::error::Error>>{
        Ok(())
    }
}

impl std::ops::Deref for Mt940Wrapper {
    type Target = MT940;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Реализация DerefMut для изменяемого доступа
impl std::ops::DerefMut for Mt940Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



#[cfg(test)]
mod tests {
    use std::fs;

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
        let result = Mt940Wrapper::read(&mut file);
        let message = match result {
            Ok(messages) => messages,
            _ => panic!("error")
        };

        //assert
        assert_eq!(Field20{reference: String::from("123456789")}, message.field_20);

        // for msg in messages {
        //     println!("Transaction ref: {}", msg.transaction_ref_no);
        //     println!("Account ID: {}", msg.account_id);
        //     println!("Opening balance: {:?}", msg.opening_balance);
        //     println!("Closing balance: {:?}", msg.closing_balance);

        //     for line in msg.statement_lines.iter() {
        //         println!("  Line: {:?}", line);
        //     }
        //     println!("---");
        // }
    }

    #[test]
    fn parsing_mt940_test_stdint() {
        //act
        let data = ":20:123456789\n:25:123456789/12345678\n:28C:00001/001\n:60F:C240930EUR12345,67\n:61:2410011001D123,45NTRFNONREF//123456789\n:86:Transfer to John Doe\n:61:2410021002C456,78NTRFNONREF//987654321\n:86:Payment from ACME Corp\n:62F:C241002EUR12679,00\n".as_bytes();
        let mut cursor = std::io::Cursor::new(data);

        //arrange
        let result = Mt940Wrapper::read(&mut cursor);
        let messages = match result {
            Ok(messages) => messages,
            _ => panic!("error")
        };
        //assert
        // for msg in messages {
        //     println!("Transaction ref: {}", msg.transaction_ref_no);
        //     println!("Account ID: {}", msg.account_id);
        //     println!("Opening balance: {:?}", msg.opening_balance);
        //     println!("Closing balance: {:?}", msg.closing_balance);

        //     for line in msg.statement_lines.iter() {
        //         println!("  Line: {:?}", line);
        //     }
        //     println!("---");
        // }
    }

    #[test]
    fn parsing_mt940_test_write() {
        //act
        let file_path = "statement.mt940";
        let f = std::fs::File::open(file_path);
        let mut file = match f {
            Ok(file) => file,
            _ => panic!("test panic.")
        };

        //arrange
        let result = Mt940Wrapper::read(&mut file);
        let message = match result {
            Ok(messages) => messages,
            _ => panic!("error")
        };
        let file_name = "output.mt940";
        let ff = std::fs::File::create(file_name);

        let mut file1 = match ff {
            Ok(file) => file,
            _ => panic!("test panic.")
        };

        //arrange
        let result = message.write(&mut file1);
        // let messages = match result {
        //     Ok(messages) => messages,
        //     _ => panic!("error")
        // };
        //assert
        // for msg in messages {
        //     println!("Transaction ref: {}", msg.transaction_ref_no);
        //     println!("Account ID: {}", msg.account_id);
        //     println!("Opening balance: {:?}", msg.opening_balance);
        //     println!("Closing balance: {:?}", msg.closing_balance);

        //     for line in msg.statement_lines.iter() {
        //         println!("  Line: {:?}", line);
        //     }
        //     println!("---");
        // }
    }
}
