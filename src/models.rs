use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Document")]
pub struct Document {
    #[serde(rename = "xmlns", default = "default_xmlns")]
    pub xmlns: String,
    #[serde(rename = "BkToCstmrStmt")]
    pub bk_to_cstmr_stmt: BkToCstmrStmt,
}

fn default_xmlns() -> String {
    "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BkToCstmrStmt {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GrpHdr,
    #[serde(rename = "Stmt")]
    pub stmt: Vec<Stmt>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrpHdr {
    #[serde(rename = "MsgId")]
    pub msg_id: MsgId,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: CreDtTm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgId {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreDtTm {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stmt {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none")]
    pub elctrnc_seq_nb: Option<u32>,
    #[serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none")]
    pub lgl_seq_nb: Option<u32>,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: CreDtTm,
    #[serde(rename = "Acct")]
    pub acct: Acct,
    #[serde(rename = "Ntry", default)]
    pub ntry: Vec<Ntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IBAN>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IBAN {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Acct {
    #[serde(rename = "Id")]
    pub id: AcctId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcctId {
    #[serde(rename = "IBAN")]
    pub iban: IBAN,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ntry {
    #[serde(rename = "Amt")]
    pub amt: f64,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: String,
    #[serde(rename = "NtryDtls")]
    pub ntry_dtls: NtryDtls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NtryDtls {
    #[serde(rename = "TxDtls")]
    pub tx_dtls: Vec<TxDtls>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxDtls {
    #[serde(rename = "Amt")]
    pub amt: f64,
}
