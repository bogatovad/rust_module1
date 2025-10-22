use swift_mt_message::messages::mt940::{MT940, MT940StatementLine};
use swift_mt_message::fields::field20::Field20;
use crate::cam_struct::*;
use swift_mt_message::fields::*;
use chrono::Utc;


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

    pub fn to_camt053(&self) -> Result<Document, Box<dyn std::error::Error>> {
        let mt940 = &self.0;
        let current_time = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let document = Document {
            bk_to_cstmr_stmt: BkToCstmrStmt {
                grp_hdr: GrpHdr {
                    msg_id: mt940.field_20.reference.clone(),
                    cre_dt_tm: current_time.clone(),
                },
                stmt: Stmt {
                    id: mt940.field_20.reference.clone(),
                    elctrnc_seq_nb: mt940.field_28c.statement_number as i32,
                    lgl_seq_nb: mt940.field_28c.sequence_number.unwrap_or(1) as i32,
                    cre_dt_tm: current_time,
                    fr_to_dt: FrToDt {
                        fr_dt_tm: format!("{}T00:00:00", mt940.field_60f.value_date),
                        to_dt_tm: format!("{}T23:59:59", mt940.field_62f.value_date),
                    },
                    acct: Acct {
                        id: AccountId {
                            iban: Some(mt940.field_25.authorisation.clone()),
                        },
                        ccy: mt940.field_60f.currency.clone(),
                        nm: "Account Holder".to_string(),
                        ownr: Ownr {
                            nm: "Account Owner".to_string(),
                            pstl_adr: PstlAdr {
                                strt_nm: Some("Street".to_string()),
                                bldg_nb: Some("123".to_string()),
                                pst_cd: Some("12345".to_string()),
                                twn_nm: Some("City".to_string()),
                                ctry: Some("XX".to_string()),
                                adr_line: None,
                            },
                            id: OwnrId {
                                org_id: OrgId {
                                    othr: Othr {
                                        id: "1234567890".to_string(),
                                        schme_nm: SchmeNm {
                                            cd: "CUST".to_string(),
                                        },
                                    },
                                },
                            },
                        },
                        svcr: Svcr {
                            fin_instn_id: FinInstnId {
                                bic: Some("XXXXXXXXXXX".to_string()),
                                nm: Some("Bank Name".to_string()),
                                pstl_adr: None,
                            },
                        },
                    },
                    balances: self.create_balances(mt940)?,
                    txs_summry: self.create_transaction_summary(mt940)?,
                    entries: self.create_entries(mt940)?,
                },
            },
        };
        
        Ok(document)
    }
    
    fn create_balances(&self, mt940: &MT940) -> Result<Vec<Bal>, Box<dyn std::error::Error>> {
        let mut balances = Vec::new();
        balances.push(Bal {
            tp: BalTp {
                cd_or_prtry: CdOrPrtry {
                    cd: "OPBD".to_string(),
                },
            },
            amt: Amount {
                ccy: mt940.field_60f.currency.clone(),
                value: format!("{:.2}", mt940.field_60f.amount),
            },
            cdt_dbt_ind: self.convert_mt940_indicator(&mt940.field_60f.debit_credit_mark)?,
            dt: BalDt {
                dt: mt940.field_60f.value_date.to_string(),
            },
        });
        balances.push(Bal {
            tp: BalTp {
                cd_or_prtry: CdOrPrtry {
                    cd: "CLBD".to_string(),
                },
            },
            amt: Amount {
                ccy: mt940.field_62f.currency.clone(),
                value: format!("{:.2}", mt940.field_62f.amount),
            },
            cdt_dbt_ind: self.convert_mt940_indicator(&mt940.field_62f.debit_credit_mark)?,
            dt: BalDt {
                dt: mt940.field_62f.value_date.to_string(),
            },
        });
        if let Some(field_64) = &mt940.field_64 {
            balances.push(Bal {
                tp: BalTp {
                    cd_or_prtry: CdOrPrtry {
                        cd: "CLAV".to_string(),
                    },
                },
                amt: Amount {
                    ccy: field_64.currency.clone(),
                    value: format!("{:.2}", field_64.amount),
                },
                cdt_dbt_ind: self.convert_mt940_indicator(&field_64.debit_credit_mark)?,
                dt: BalDt {
                    dt: field_64.value_date.to_string(),
                },
            });
        }
        
        Ok(balances)
    }
    
    fn create_transaction_summary(&self, mt940: &MT940) -> Result<Option<TxsSummry>, Box<dyn std::error::Error>> {
        let total_entries = mt940.statement_lines.len();
        let (credit_count, credit_sum) = self.calculate_credit_transactions(mt940);
        let (debit_count, debit_sum) = self.calculate_debit_transactions(mt940);
        
        let net_amount = credit_sum - debit_sum;
        let net_indicator = if net_amount >= 0.0 { "CRDT" } else { "DBIT" };
        
        Ok(Some(TxsSummry {
            ttl_ntries: TtlNtries {
                nb_of_ntries: total_entries.to_string(),
                ttl_net_ntry_amt: format!("{:.2}", net_amount.abs()),
                cdt_dbt_ind: net_indicator.to_string(),
            },
            ttl_cdt_ntries: TtlCdtNtries {
                nb_of_ntries: credit_count.to_string(),
                sum: format!("{:.2}", credit_sum),
            },
            ttl_dbt_ntries: TtlDbtNtries {
                nb_of_ntries: debit_count.to_string(),
                sum: format!("{:.2}", debit_sum),
            },
        }))
    }
    
    fn create_entries(&self, mt940: &MT940) -> Result<Vec<Ntry>, Box<dyn std::error::Error>> {
        let mut entries = Vec::new();
        
        for (index, line) in mt940.statement_lines.iter().enumerate() {
            let entry_ref = (index + 1).to_string();
            
            let entry = Ntry {
                ntry_ref: entry_ref.clone(),
                amt: Amount {
                    ccy: mt940.field_60f.currency.clone(),
                    value: format!("{:.2}", line.field_61.amount),
                },
                cdt_dbt_ind: self.convert_mt940_indicator(&line.field_61.debit_credit_mark)?,
                sts: "BOOK".to_string(),
                bookg_dt: BookgDt {
                    dt: line.field_61.value_date.to_string(),
                },
                val_dt: ValDt {
                    dt: line.field_61.value_date.to_string(),
                },
                acct_svcr_ref: line.field_61.customer_reference.clone(),
                bk_tx_cd: BkTxCd {
                    domn: Domn {
                        cd: "PMNT".to_string(),
                        fmly: Fmly {
                            cd: self.determine_family_code(&line.field_61.transaction_type)?,
                            sub_fmly_cd: "STDO".to_string(),
                        },
                    },
                    prtry: Prtry {
                        cd: line.field_61.transaction_type.clone(),
                        issr: "BANK".to_string(),
                    },
                },
                ntry_dtls: NtryDtls {
                    tx_dtls: vec![self.create_transaction_details(&line.field_61, line.field_86.as_ref())?],
                },
            };
            
            entries.push(entry);
        }
        
        Ok(entries)
    }
    
    fn create_transaction_details(&self, field_61: &Field61, field_86: Option<&Field86>) -> Result<TxDtls, Box<dyn std::error::Error>> {
        let mut refs = Refs {
            end_to_end_id: field_61.bank_reference.clone(),
        };
        
        // Извлекаем EndToEndId из narrative если возможно
        if let Some(narrative_field) = field_86 {
            for line in &narrative_field.narrative {
                if line.contains("EndToEndId:") {
                    if let Some(id) = line.split("EndToEndId:").nth(1) {
                        refs.end_to_end_id = Some(id.trim().to_string());
                    }
                }
            }
        }
        
        Ok(TxDtls {
            refs: Some(refs),
            amt_dtls: Some(AmtDtls {
                tx_amt: TxAmt {
                    amt: Amount {
                        ccy: "EUR".to_string(),
                        value: format!("{:.2}", field_61.amount),
                    },
                },
            }),
            rltd_pties: None,
            rltd_dts: Some(RltdDts {
                accptnc_dt_tm: format!("{}T00:00:00", field_61.value_date),
            }),
        })
    }
    
    fn calculate_credit_transactions(&self, mt940: &MT940) -> (usize, f64) {
        let credit_lines: Vec<&MT940StatementLine> = mt940.statement_lines
            .iter()
            .filter(|line| line.field_61.debit_credit_mark == "C")
            .collect();
        
        let total_credit: f64 = credit_lines.iter()
            .map(|line| line.field_61.amount)
            .sum();
            
        (credit_lines.len(), total_credit)
    }
    
    fn calculate_debit_transactions(&self, mt940: &MT940) -> (usize, f64) {
        let debit_lines: Vec<&MT940StatementLine> = mt940.statement_lines
            .iter()
            .filter(|line| line.field_61.debit_credit_mark == "D")
            .collect();
        
        let total_debit: f64 = debit_lines.iter()
            .map(|line| line.field_61.amount)
            .sum();
            
        (debit_lines.len(), total_debit)
    }
    
    fn convert_mt940_indicator(&self, indicator: &str) -> Result<String, Box<dyn std::error::Error>> {
        match indicator {
            "D" => Ok("DBIT".to_string()),
            "C" => Ok("CRDT".to_string()),
            _ => Err(format!("Invalid MT940 debit/credit indicator: {}", indicator).into()),
        }
    }
    
    fn determine_family_code(&self, transaction_type: &str) -> Result<String, Box<dyn std::error::Error>> {
        match transaction_type {
            "CRED" => Ok("RCDT".to_string()),
            "DEBT" => Ok("ICDT".to_string()),
            "CARD" => Ok("MCRD".to_string()),
            _ => Ok("PMNT".to_string()),
        }
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

        let mut doc = message.to_camt053().unwrap();


        let mut f = std::fs::File::create("test.xml").unwrap();
        doc.write(&mut f);

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
