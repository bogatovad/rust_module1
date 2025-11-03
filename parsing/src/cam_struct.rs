use serde::{Deserialize, Serialize};

/// This is the stuct which implements data in mt940 formats
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Document {
    pub bk_to_cstmr_stmt: BkToCstmrStmt,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BkToCstmrStmt {
    pub grp_hdr: GrpHdr,
    pub stmt: Stmt,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GrpHdr {
    pub msg_id: String,
    pub cre_dt_tm: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stmt {
    pub id: String,
    pub elctrnc_seq_nb: i32,
    pub lgl_seq_nb: i32,
    pub cre_dt_tm: String,
    pub fr_to_dt: FrToDt,
    pub acct: Acct,
    #[serde(rename = "Bal")]
    pub balances: Vec<Bal>,
    pub txs_summry: Option<TxsSummry>,
    #[serde(rename = "Ntry")]
    pub entries: Vec<Ntry>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TxsSummry {
    pub ttl_ntries: TtlNtries,
    pub ttl_cdt_ntries: TtlCdtNtries,
    pub ttl_dbt_ntries: TtlDbtNtries,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TtlNtries {
    pub nb_of_ntries: String,
    pub ttl_net_ntry_amt: String,
    pub cdt_dbt_ind: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TtlCdtNtries {
    pub nb_of_ntries: String,
    pub sum: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TtlDbtNtries {
    pub nb_of_ntries: String,
    pub sum: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FrToDt {
    pub fr_dt_tm: String,
    pub to_dt_tm: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Acct {
    pub id: AccountId,
    pub ccy: String,
    pub nm: String,
    pub ownr: Ownr, 
    pub svcr: Svcr,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ownr {
    pub nm: String,
    pub pstl_adr: PstlAdr,
    pub id: OwnrId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PstlAdr {
    pub strt_nm: Option<String>,
    pub bldg_nb: Option<String>,
    pub pst_cd: Option<String>,
    pub twn_nm: Option<String>,
    pub ctry: Option<String>,
    #[serde(rename = "AdrLine")]
    pub adr_line: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OwnrId {
    pub org_id: OrgId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrgId {
    pub othr: Othr,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Othr {
    pub id: String,
    pub schme_nm: SchmeNm,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SchmeNm {
    pub cd: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Svcr {
    pub fin_instn_id: FinInstnId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FinInstnId {
    #[serde(rename = "BIC")]
    pub bic: Option<String>,
    pub nm: Option<String>,
    pub pstl_adr: Option<PstlAdr>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountId {
    #[serde(rename = "IBAN")]
    pub iban: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountIdSecond {
    #[serde(rename = "Othr")]
    pub othr: Option<Id>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Id {
    pub id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bal {
    pub tp: BalTp,
    pub amt: Amount,
    pub cdt_dbt_ind: String,
    pub dt: BalDt,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalTp {
    pub cd_or_prtry: CdOrPrtry,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CdOrPrtry {
    pub cd: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Amount {
    #[serde(rename = "@Ccy")]
    pub ccy: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BalDt {
    pub dt: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ntry {
    pub ntry_ref: String,
    pub amt: Amount,
    pub cdt_dbt_ind: String,
    pub sts: String,
    pub bookg_dt: BookgDt,
    pub val_dt: ValDt,
    pub acct_svcr_ref: String,
    pub bk_tx_cd: BkTxCd,
    //pub addtl_inf_ind: AddtlInfInd,
    pub ntry_dtls: NtryDtls
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddtlInfInd {
    pub msg_nm_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BookgDt {
    pub dt: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValDt {
    pub dt: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BkTxCd {
    pub domn: Domn,
    pub prtry: Prtry,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Domn {
    pub cd: String,
    pub fmly: Fmly,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fmly {
    pub cd: String,
    pub sub_fmly_cd: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Prtry {
    pub cd: String,
    pub issr: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NtryDtls {
    #[serde(rename = "TxDtls")]
    pub tx_dtls: Vec<TxDtls>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TxDtls {
    pub refs: Option<Refs>,
    pub amt_dtls: Option<AmtDtls>,
    pub rltd_pties: Option<RltdPties>,
    pub rltd_dts: Option<RltdDts>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Refs {
    pub end_to_end_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AmtDtls {
    pub tx_amt: TxAmt,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TxAmt {
    pub amt: Amount,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RltdPties {
    pub cdtr_acct: Option<AccountPties>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RltdDts {
    pub accptnc_dt_tm: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    pub id: AccountId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountPties  {
    pub id: AccountIdSecond,
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct AccountId {
//     #[serde(rename = "Othr")]
//     pub other: Option<OtherAccountId>,
//     #[serde(rename = "IBAN")]
//     pub iban: Option<String>,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OtherAccountId {
    pub id: String,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Amount {
//     #[serde(rename = "@Ccy")]
//     pub ccy: String,
//     #[serde(rename = "$value")]
//     pub value: String,
// }