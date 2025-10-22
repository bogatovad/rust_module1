use crate::cam_struct::{Document, Bal, Ntry, BkTxCd};
use crate::mt940_parsing::Mt940Wrapper;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use swift_mt_message::messages::mt940::{MT940, MT940StatementLine};
use swift_mt_message::fields::{Field61, Field86, Field20, Field25NoOption, Field28C, Field60F, Field62F, Field64};
use chrono::{NaiveDate, NaiveDateTime};

impl Document{
    pub fn read<R: std::io::Read>(reader: &mut R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut xml_content = String::new();
        reader.read_to_string(&mut xml_content)?;
        let document: Document = from_str(&xml_content)?;
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
        let field_20 = Field20 { reference: stmt.id.clone() };
        
        // Field 25: Account Identification
        let iban = stmt.acct.id.iban.as_ref()
            .ok_or("IBAN is required for MT940 conversion")?;
        let field_25 = Field25NoOption { authorisation: iban.clone() };
        
        // Field 28C: Statement Number/Sequence Number
        let field_28c = Field28C {
            statement_number: stmt.elctrnc_seq_nb as u32,
            sequence_number: Some(stmt.lgl_seq_nb as u32),
        };
        
        // Find appropriate balances
        let opening_balance = self.find_balance_by_type("OPBD")
            .or_else(|| self.find_balance_by_type("OPAV"))
            .ok_or("No opening balance found")?;
            
        let closing_balance = self.find_balance_by_type("CLBD")
            .or_else(|| self.find_balance_by_type("CLAV"))
            .ok_or("No closing balance found")?;
        
        // Field 60F: Opening Balance
        let field_60f = Field60F {
            value_date: self.parse_date(&opening_balance.dt.dt)?,
            debit_credit_mark: self.convert_debit_credit(&opening_balance.cdt_dbt_ind)?,
            currency: opening_balance.amt.ccy.clone(),
            amount: self.parse_amount(&opening_balance.amt.value)?,
        };
        
        // Field 62F: Closing Balance
        let field_62f = Field62F {
            value_date: self.parse_date(&closing_balance.dt.dt)?,
            debit_credit_mark: self.convert_debit_credit(&closing_balance.cdt_dbt_ind)?,
            currency: closing_balance.amt.ccy.clone(),
            amount: self.parse_amount(&closing_balance.amt.value)?,
        };
        
        // Field 64: Available Balance
        let field_64 = self.find_balance_by_type("CLAV")
            .map(|bal| Field64 {
                value_date: self.parse_date(&bal.dt.dt).unwrap_or_else(|_| NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()),
                debit_credit_mark: self.convert_debit_credit(&bal.cdt_dbt_ind).unwrap_or("C".to_string()),
                currency: bal.amt.ccy.clone(),
                amount: self.parse_amount(&bal.amt.value).unwrap_or(0.0),
            });
        
        // Statement Lines
        let mut statement_lines = Vec::new();
        for entry in &stmt.entries {
            if let Some(stmt_line) = self.convert_entry_to_statement_line(entry)? {
                statement_lines.push(stmt_line);
            }
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

    fn find_balance_by_type(&self, balance_type: &str) -> Option<&Bal> {
        self.bk_to_cstmr_stmt.stmt.balances
            .iter()
            .find(|bal| bal.tp.cd_or_prtry.cd == balance_type)
    }

    fn convert_entry_to_statement_line(&self, entry: &Ntry) -> Result<Option<MT940StatementLine>, Box<dyn std::error::Error>> {
        // Field 61: Transaction Details
        let value_date = self.parse_date(&entry.val_dt.dt)?;
        let entry_date = self.parse_date(&entry.bookg_dt.dt)
            .map(|d| d.format("%m%d").to_string())
            .ok();
        
        let field_61 = Field61 {
            value_date,
            entry_date,
            debit_credit_mark: self.convert_debit_credit(&entry.cdt_dbt_ind)?,
            amount: self.parse_amount(&entry.amt.value)?,
            transaction_type: self.determine_transaction_type(&entry.bk_tx_cd)?,
            customer_reference: entry.acct_svcr_ref.clone(),
            bank_reference: Some(entry.ntry_ref.clone()),
            funds_code: None,
            supplementary_details: None,
        };
        
        // Field 86: Narrative
        let narrative = self.build_narrative(entry)?;
        let field_86 = if !narrative.is_empty() {
            Some(Field86 { narrative })
        } else {
            None
        };
        
        Ok(Some(MT940StatementLine {
            field_61,
            field_86,
        }))
    }

    fn build_narrative(&self, entry: &Ntry) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut narrative = Vec::new();
        narrative.push(format!("{}", entry.bk_tx_cd.prtry.cd));
        
        if let Some(tx_dtls) = entry.ntry_dtls.tx_dtls.first() {
            if let Some(refs) = &tx_dtls.refs {
                if let Some(end_to_end_id) = &refs.end_to_end_id {
                    if end_to_end_id != "NOTPROVIDED" {
                        narrative.push(format!("EndToEndId: {}", end_to_end_id));
                    }
                }
            }
            if let Some(rltd_pties) = &tx_dtls.rltd_pties {
                if let Some(cdtr_acct) = &rltd_pties.cdtr_acct {
                    if let Some(othr) = &cdtr_acct.id.othr {
                        if let Some(id) = othr.id {
                            narrative.push(format!("Creditor: {}", id));
                        }
                    }
                }
            }
            if let Some(rltd_dts) = &tx_dtls.rltd_dts {
                narrative.push(format!("Acceptance: {}", rltd_dts.accptnc_dt_tm));
            }
        }
        narrative.push(format!("Status: {}", entry.sts));
        
        Ok(narrative)
    }

    fn determine_transaction_type(&self, bk_tx_cd: &BkTxCd) -> Result<String, Box<dyn std::error::Error>> {
        match bk_tx_cd.domn.fmly.cd.as_str() {
            "RCDT" => Ok("CRED".to_string()), // Credit transfer
            "ICDT" => Ok("DEBT".to_string()), // Debit transfer
            "MCRD" => Ok("CARD".to_string()), // Card transaction
            _ => Ok("NTRF".to_string()), // Default: transfer
        }
    }

    fn parse_date(&self, date_str: &str) -> Result<NaiveDate, Box<dyn std::error::Error>> {
        if let Ok(datetime) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S") {
            return Ok(datetime.date());
        }
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            return Ok(date);
        }
        Err(format!("Invalid date format: {}", date_str).into())
    }

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