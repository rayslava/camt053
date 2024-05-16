use quick_xml::se::to_writer;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;

#[derive(Serialize)]
#[serde(rename = "Document")]
pub struct Document {
    #[serde(rename = "xmlns")]
    pub xmlns: String,
    #[serde(rename = "BkToCstmrStmt")]
    pub statement: BankToCustomerStatement,
}

#[derive(Serialize)]
#[serde(rename = "BkToCstmrStmt")]
pub struct BankToCustomerStatement {
    #[serde(rename = "GrpHdr")]
    pub group_header: GroupHeader,
    #[serde(rename = "Stmt")]
    pub statement: Statement,
}

#[derive(Serialize)]
pub struct GroupHeader {
    #[serde(rename = "MsgId")]
    pub message_id: String,
    #[serde(rename = "CreDtTm")]
    pub creation_date_time: String,
}

#[derive(Serialize)]
pub struct Statement {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "CreDtTm")]
    pub creation_date_time: String,
    #[serde(rename = "Acct")]
    pub account: Account,
    #[serde(rename = "Bal")]
    pub balance: Balance,
    #[serde(rename = "Ntry")]
    pub entries: Vec<Entry>,
}

#[derive(Serialize)]
pub struct Account {
    #[serde(rename = "Id")]
    pub id: AccountId,
}

#[derive(Serialize)]
pub struct AccountId {
    #[serde(rename = "IBAN")]
    pub iban: String,
}

#[derive(Serialize)]
pub struct Balance {
    #[serde(rename = "Tp")]
    pub type_: BalanceType,
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub credit_debit_indicator: String,
    #[serde(rename = "Dt")]
    pub date: BalanceDate,
}

#[derive(Serialize)]
pub struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    pub code_or_proprietary: CodeOrProprietary,
}

#[derive(Serialize)]
pub struct CodeOrProprietary {
    #[serde(rename = "Cd")]
    pub code: String,
}

#[derive(Serialize)]
pub struct Amount {
    #[serde(rename = "Ccy")]
    pub currency: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize)]
pub struct BalanceDate {
    #[serde(rename = "Dt")]
    pub date: String,
}

#[derive(Serialize)]
pub struct Entry {
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub credit_debit_indicator: String,
    #[serde(rename = "Sts")]
    pub status: String,
    #[serde(rename = "BookgDt")]
    pub booking_date: EntryDate,
    #[serde(rename = "ValDt")]
    pub value_date: EntryDate,
    #[serde(rename = "AcctSvcrRef")]
    pub account_servicer_reference: String,
    #[serde(rename = "BkTxCd")]
    pub bank_transaction_code: BankTransactionCode,
    #[serde(rename = "NtryDtls")]
    pub entry_details: EntryDetails,
}

#[derive(Serialize)]
pub struct EntryDate {
    #[serde(rename = "Dt")]
    pub date: String,
}

#[derive(Serialize)]
pub struct BankTransactionCode {
    #[serde(rename = "Domn")]
    pub domain: Domain,
}

#[derive(Serialize)]
pub struct Domain {
    #[serde(rename = "Cd")]
    pub code: String,
    #[serde(rename = "Fmly")]
    pub family: Family,
}

#[derive(Serialize)]
pub struct Family {
    #[serde(rename = "Cd")]
    pub code: String,
    #[serde(rename = "SubFmlyCd")]
    pub sub_family_code: String,
}

#[derive(Serialize)]
pub struct EntryDetails {
    #[serde(rename = "TxDtls")]
    pub transaction_details: Vec<TransactionDetails>,
}

#[derive(Serialize)]
pub struct TransactionDetails {
    #[serde(rename = "Refs")]
    pub references: References,
    #[serde(rename = "AmtDtls")]
    pub amount_details: AmountDetails,
    #[serde(rename = "RltdPties")]
    pub related_parties: RelatedParties,
    #[serde(rename = "RmtInf")]
    pub remittance_information: RemittanceInformation,
}

#[derive(Serialize)]
pub struct References {
    #[serde(rename = "AcctSvcrRef")]
    pub account_servicer_reference: String,
}

#[derive(Serialize)]
pub struct AmountDetails {
    #[serde(rename = "TxAmt")]
    pub transaction_amount: Amount,
}

#[derive(Serialize)]
pub struct RelatedParties {
    #[serde(rename = "Dbtr")]
    pub debtor: Debtor,
    #[serde(rename = "Cdtr")]
    pub creditor: Creditor,
}

#[derive(Serialize)]
pub struct Debtor {
    #[serde(rename = "Nm")]
    pub name: String,
    #[serde(rename = "PstlAdr")]
    pub postal_address: PostalAddress,
}

#[derive(Serialize)]
pub struct Creditor {
    #[serde(rename = "Nm")]
    pub name: String,
    #[serde(rename = "PstlAdr")]
    pub postal_address: PostalAddress,
}

#[derive(Serialize)]
pub struct PostalAddress {
    #[serde(rename = "AdrLine")]
    pub address_lines: Vec<String>,
}

#[derive(Serialize)]
pub struct RemittanceInformation {
    #[serde(rename = "Ustrd")]
    pub unstructured: String,
}

pub struct Transaction {
    pub amount: Amount,
    pub credit_debit_indicator: String,
    pub status: String,
    pub booking_date: EntryDate,
    pub value_date: EntryDate,
    pub account_servicer_reference: String,
    pub bank_transaction_code: BankTransactionCode,
    pub entry_details: Vec<TransactionDetails>,
}

pub fn generate_camt053(
    file_path: &str,
    message_id: &str,
    creation_date_time: &str,
    statement_id: &str,
    statement_creation_date_time: &str,
    iban: &str,
    balance_code: &str,
    balance_amount: &str,
    balance_currency: &str,
    balance_credit_debit_indicator: &str,
    balance_date: &str,
    transactions: Vec<Transaction>,
) -> Result<(), Box<dyn std::error::Error>> {
    let document = Document {
        xmlns: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.02".to_string(),
        statement: BankToCustomerStatement {
            group_header: GroupHeader {
                message_id: message_id.to_string(),
                creation_date_time: creation_date_time.to_string(),
            },
            statement: Statement {
                id: statement_id.to_string(),
                creation_date_time: statement_creation_date_time.to_string(),
                account: Account {
                    id: AccountId {
                        iban: iban.to_string(),
                    },
                },
                balance: Balance {
                    type_: BalanceType {
                        code_or_proprietary: CodeOrProprietary {
                            code: balance_code.to_string(),
                        },
                    },
                    amount: Amount {
                        currency: balance_currency.to_string(),
                        value: balance_amount.to_string(),
                    },
                    credit_debit_indicator: balance_credit_debit_indicator.to_string(),
                    date: BalanceDate {
                        date: balance_date.to_string(),
                    },
                },
                entries: transactions
                    .into_iter()
                    .map(|t| Entry {
                        amount: t.amount,
                        credit_debit_indicator: t.credit_debit_indicator,
                        status: t.status,
                        booking_date: t.booking_date,
                        value_date: t.value_date,
                        account_servicer_reference: t.account_servicer_reference,
                        bank_transaction_code: t.bank_transaction_code,
                        entry_details: EntryDetails {
                            transaction_details: t.entry_details,
                        },
                    })
                    .collect(),
            },
        },
    };

    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);

    to_writer(writer, &document)?;
    Ok(())
}
