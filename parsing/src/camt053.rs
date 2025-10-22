use crate::cam_struct::Document;
use crate::mt940_parsing::Mt940Wrapper;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use swift_mt_message::messages::mt940::{MT940, MT940StatementLine};
use swift_mt_message::fields::{Field61, Field86, Field20, Field25NoOption, Field28C, Field60F, Field62F, Field64, Field65};
use chrono::{NaiveDate, NaiveDateTime};

impl Document{
    pub fn read<R: std::io::Read>(reader: &mut R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut xml_content = String::new();
        reader.read_to_string(&mut xml_content)?;
        let document: Document = from_str(&xml_content)?;
        //print!("ITEM!! {:?}", document);
        Ok(document)
    }

    pub fn write<W: std::io::Write>(&mut self, writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let xml_content = to_string(&self)?;
        writer.write_all(xml_content.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    pub fn to_mt940(&self) -> Result<Mt940Wrapper, Box<dyn std::error::Error>> {
        let stmt = &self.bk_to_cstmr_stmt.stmt;
        
        // Field 20: Transaction Reference Number
        let field_20 = Field20{reference: stmt.id.clone()};
        
        // Field 25: Account Identification
        let iban = stmt.acct.id.iban.as_ref().ok_or("IBAN is required for MT940 conversion")?;
        let field_25 = Field25NoOption{authorisation: iban.clone()};
        let seq_number = stmt.lgl_seq_nb.clone() as u32;

        // Field 28C: Statement Number/Sequence Number
        let field_28c = Field28C{
            statement_number: stmt.elctrnc_seq_nb.clone() as u32, 
            sequence_number: Some(seq_number)
        };
        
        // Field 60F: Opening Balance - используем первый доступный баланс
        let opening_balance = stmt.balances.first().ok_or("No balances found for opening balance")?;
        
        let field_60f = Field60F {
            value_date: NaiveDate::from_ymd_opt(2023, 12, 31).expect("Valid date"),
            debit_credit_mark: "5".to_string(),
            currency: opening_balance.amt.ccy.clone(),
            amount: self.parse_amount(&opening_balance.amt.value)?,
        };
        
        // Field 62F: Closing Balance - используем последний доступный баланс
        let closing_balance = stmt.balances.last()
            .ok_or("No balances found for closing balance")?;
        
        
        let field_62f = Field62F {
            value_date: NaiveDate::from_ymd_opt(2023, 12, 31).expect("Valid date"),
            debit_credit_mark: "5".to_string(),
            currency: closing_balance.amt.ccy.clone(),
            amount: self.parse_amount(&closing_balance.amt.value)?,
        };
        
        // Field 64: Available Balance
        let field_64 = stmt.balances.iter()
            .find(|bal| bal.tp.cd_or_prtry.cd == "CLAV")
            .map(|bal| Field64 {
                value_date: NaiveDate::from_ymd_opt(2023, 12, 31).expect("Valid date"),
                debit_credit_mark: "5".to_string(),
                currency: bal.amt.ccy.clone(),
                amount: self.parse_amount(&bal.amt.value).unwrap_or(0.0),
            });
        
        let narrative: Vec<String> = vec!["sada".to_string(), "dsgg".to_string()];

        // Statement Lines
        let mut statement_lines: Vec<MT940StatementLine> = Vec::new();

        for entry in &stmt.entries {
            statement_lines.push(MT940StatementLine{
                field_61: Field61 { supplementary_details: Some("tt".to_string()), bank_reference: Some("test".to_string()), funds_code: Some('f'), value_date: NaiveDate::from_ymd_opt(2023, 12, 31).expect("Valid date"), entry_date: Some("2023-10-10".to_string()), debit_credit_mark: "C".to_string(), amount: 34.44, transaction_type: "FGTD".to_string(), customer_reference: "DFRS".to_string()},
                field_86: Some(Field86 { narrative: narrative.clone()})
            
            });
        }
        
        Ok(Mt940Wrapper(MT940 {
            field_20,
            field_21: None,
            field_25,
            field_28c,
            field_60f,
            statement_lines,
            field_62f,
            field_64,
            field_65: None,
        }))
    }

    // Вспомогательные функции
    fn convert_debit_credit(&self, indicator: &str) -> Result<String, Box<dyn std::error::Error>> {
        match indicator {
            "DBIT" => Ok("D".to_string()),
            "CRDT" => Ok("C".to_string()),
            _ => Err(format!("Invalid debit/credit indicator: {}", indicator).into()),
        }
    }

    fn parse_amount(&self, amount_str: &str) -> Result<f64, Box<dyn std::error::Error>> {
        amount_str.parse::<f64>().map_err(|e| e.into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_camt053_test_read_file() {
        //arrange
        let mut file = std::fs::File::open("camt053.xml").unwrap();

        //act
        let mut message = Document::read(&mut file).unwrap();
        
        let mut new_f = std::fs::File::create("new.xml").unwrap();
        let _ = message.write(&mut new_f);

        let mt940 = message.to_mt940().unwrap();
        
        let mut file_1 = std::fs::File::create("out.mt940").unwrap();

        let _ = mt940.write(&mut file_1);



        // //assert
        // assert_eq!(message.xmlns, Some("urn:swift:xsd:$ahV10".to_string()));
        // assert_eq!(message.xmlns_xsi, Some("http://www.w3.org/2001/XMLSchema-instance".to_string()));
        // assert_eq!(message.app_hdr.biz_msg_idr, "MSG20251020001");
        // assert_eq!(message.app_hdr.msg_def_idr, "pacs.008.001.08");
        // assert_eq!(message.app_hdr.biz_svc, "urn:service:cbpr");
        // assert_eq!(message.app_hdr.cre_dt, "2025-10-20T10:00:00Z");
        // if let Some(fr_fi_id) = &message.app_hdr.fr.fi_id {
        //     assert_eq!(fr_fi_id.fin_instn_id.bicfi, "AAAADEFFXXX");
        // }
        // if let Some(to_fi_id) = &message.app_hdr.to.fi_id {
        //     assert_eq!(to_fi_id.fin_instn_id.bicfi, "BBBBUS33XXX");
        // }
    }

    // #[test]
    // fn parsing_camt053_test_write_file() {
    //     //arrange
    //     let mut file = std::fs::File::open("camt053.xml").unwrap();
    //     let mut w_file = std::fs::File::create("output.xml").unwrap();

    //     //act
    //     let mut message = MxMessageWrapper::read(&mut file).unwrap();
    //     let _ = message.write(&mut w_file);
    // }
}