use serde_xml_rs::from_str;
use iso_20022_sdk::prelude::Message;
use iso_20022_sdk::prelude::*;
use iso_20022_sdk::documents::Document as Camt053Document;
use mx_message::mx_envelope::MxMessage;


pub fn read<R: std::io::Read>(reader: &mut R) -> Result<(), Box<dyn std::error::Error>> {
    let mut xml_content = String::new();
    reader.read_to_string(&mut xml_content)?;
    //println!("XML!! {}", &xml_content);
    //let mut doc = Document::from_namespace("remt.001.001.01")?;
    //let message = Message::<Camt053Document>::from_xml(&xml_content);
    
    let message = MxMessage::from_xml(&xml_content);
    println!("MESSAGE!! {:?}", message);
    
    // Парсинг XML в структуру Document
    //let document: Document = from_str(&xml_content)?;
    //let document: () = from_str(&mut xml_content).unwrap();
    //println!("{}", document);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_csv_test_stdin() {
        //arrange
        let mut file = std::fs::File::open("camt053.xml").unwrap();
        read(&mut file);
    }
}