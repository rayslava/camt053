use camt053::models::*;
use camt053::serialize::generate_camt053;
use quick_xml::de::from_str;
use pretty_assertions::assert_eq;
use std::fs::File;
use std::io::Write;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct TestDocument {
    #[serde(rename = "xmlns", default = "default_xmlns")]
    pub xmlns: String,
    #[serde(rename = "BkToCstmrStmt")]
    bk_to_cstmr_stmt: TestBkToCstmrStmt,
}

fn default_xmlns() -> String {
    "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02".to_string()
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestBkToCstmrStmt {
    #[serde(rename = "GrpHdr")]
    grp_hdr: TestGrpHdr,
    #[serde(rename = "Stmt")]
    stmt: Vec<TestStmt>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestGrpHdr {
    #[serde(rename = "MsgId")]
    msg_id: TestMsgId,
    #[serde(rename = "CreDtTm")]
    cre_dt_tm: TestCreDtTm,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestMsgId {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestCreDtTm {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestStmt {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none")]
    elctrnc_seq_nb: Option<u32>,
    #[serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none")]
    lgl_seq_nb: Option<u32>,
    #[serde(rename = "CreDtTm")]
    cre_dt_tm: TestCreDtTm,
    #[serde(rename = "Acct")]
    acct: TestAcct,
    #[serde(rename = "Ntry", default)]
    ntry: Vec<TestNtry>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestId {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    iban: Option<TestIBAN>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    othr: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestIBAN {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestAcct {
    #[serde(rename = "Id")]
    id: TestAcctId,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestAcctId {
    #[serde(rename = "IBAN")]
    iban: TestIBAN,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestNtry {
    #[serde(rename = "Amt")]
    amt: f64,
    #[serde(rename = "CdtDbtInd")]
    cdt_dbt_ind: String,
    #[serde(rename = "NtryDtls")]
    ntry_dtls: TestNtryDtls,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestNtryDtls {
    #[serde(rename = "TxDtls")]
    tx_dtls: Vec<TestTxDtls>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestTxDtls {
    #[serde(rename = "Amt")]
    amt: f64,
}

#[test]
fn test_generate_camt053() {
    let doc = Document {
        xmlns: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02".to_string(),
        bk_to_cstmr_stmt: BkToCstmrStmt {
            grp_hdr: GrpHdr {
                msg_id: MsgId { value: "msg123".to_string() },
                cre_dt_tm: CreDtTm { value: "2024-05-16T16:05:00".to_string() },
            },
            stmt: vec![Stmt {
                id: "01".to_string(),
                elctrnc_seq_nb: None,
                lgl_seq_nb: None,
                cre_dt_tm: CreDtTm { value: "2024-05-16T16:05:00".to_string() },
                acct: Acct {
                    id: AcctId {
                        iban: IBAN { value: "DE89370400440532013000".to_string() },
                    },
                },
                ntry: vec![
                    Ntry {
                        amt: 1000.0,
                        cdt_dbt_ind: "CRDT".to_string(),
                        ntry_dtls: NtryDtls {
                            tx_dtls: vec![
                                TxDtls { amt: 600.0 },
                                TxDtls { amt: 400.0 },
                            ],
                        },
                    },
                    Ntry {
                        amt: 1500.0,
                        cdt_dbt_ind: "DBIT".to_string(),
                        ntry_dtls: NtryDtls {
                            tx_dtls: vec![
                                TxDtls { amt: 500.0 },
                                TxDtls { amt: 500.0 },
                                TxDtls { amt: 500.0 },
                            ],
                        },
                    },
                ],
            }],
        },
    };

    let xml = generate_camt053(&doc).unwrap();
    println!("{}", xml);

    let expected_doc = TestDocument {
        xmlns: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02".to_string(),
        bk_to_cstmr_stmt: TestBkToCstmrStmt {
            grp_hdr: TestGrpHdr {
                msg_id: TestMsgId { value: "msg123".to_string() },
                cre_dt_tm: TestCreDtTm { value: "2024-05-16T16:05:00".to_string() },
            },
            stmt: vec![TestStmt {
                id: "01".to_string(),
                elctrnc_seq_nb: None,
                lgl_seq_nb: None,
                cre_dt_tm: TestCreDtTm { value: "2024-05-16T16:05:00".to_string() },
                acct: TestAcct {
                    id: TestAcctId {
                        iban: TestIBAN { value: "DE89370400440532013000".to_string() },
                    },
                },
                ntry: vec![
                    TestNtry {
                        amt: 1000.0,
                        cdt_dbt_ind: "CRDT".to_string(),
                        ntry_dtls: TestNtryDtls {
                            tx_dtls: vec![
                                TestTxDtls { amt: 600.0 },
                                TestTxDtls { amt: 400.0 },
                            ],
                        },
                    },
                    TestNtry {
                        amt: 1500.0,
                        cdt_dbt_ind: "DBIT".to_string(),
                        ntry_dtls: TestNtryDtls {
                            tx_dtls: vec![
                                TestTxDtls { amt: 500.0 },
                                TestTxDtls { amt: 500.0 },
                                TestTxDtls { amt: 500.0 },
                            ],
                        },
                    },
                ],
            }],
        },
    };

    let parsed_doc: TestDocument = from_str(&xml).unwrap();
    assert_eq!(parsed_doc, expected_doc);

    // Write the generated XML to a file
    let mut file = File::create("result.xml").expect("Unable to create file");
    file.write_all(xml.as_bytes()).expect("Unable to write data");
}
