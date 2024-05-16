use camt053::{
    generate_camt053, Amount, AmountDetails, BankTransactionCode, Creditor, Debtor, Domain,
    EntryDate, Family, PostalAddress, References, RelatedParties, RemittanceInformation,
    Transaction, TransactionDetails,
};
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

#[test]
fn test_generate_camt053() {
    let file_path = "test_output.xml";

    let transactions = vec![
        Transaction {
            amount: Amount {
                currency: "EUR".to_string(),
                value: "-100.00".to_string(),
            },
            credit_debit_indicator: "DBIT".to_string(),
            status: "BOOK".to_string(),
            booking_date: EntryDate {
                date: "2023-05-14".to_string(),
            },
            value_date: EntryDate {
                date: "2023-05-14".to_string(),
            },
            account_servicer_reference: "Ref12345".to_string(),
            bank_transaction_code: BankTransactionCode {
                domain: Domain {
                    code: "PMNT".to_string(),
                    family: Family {
                        code: "ICDT".to_string(),
                        sub_family_code: "DMCT".to_string(),
                    },
                },
            },
            entry_details: vec![
                TransactionDetails {
                    references: References {
                        account_servicer_reference: "Ref67890".to_string(),
                    },
                    amount_details: AmountDetails {
                        transaction_amount: Amount {
                            currency: "EUR".to_string(),
                            value: "-50.00".to_string(),
                        },
                    },
                    related_parties: RelatedParties {
                        debtor: Debtor {
                            name: "John Doe".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "123 Main Street".to_string(),
                                    "City, Country".to_string(),
                                ],
                            },
                        },
                        creditor: Creditor {
                            name: "Jane Smith".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "456 Secondary Street".to_string(),
                                    "Another City, Country".to_string(),
                                ],
                            },
                        },
                    },
                    remittance_information: RemittanceInformation {
                        unstructured: "Payment for invoice #1234".to_string(),
                    },
                },
                TransactionDetails {
                    references: References {
                        account_servicer_reference: "Ref67891".to_string(),
                    },
                    amount_details: AmountDetails {
                        transaction_amount: Amount {
                            currency: "EUR".to_string(),
                            value: "-50.00".to_string(),
                        },
                    },
                    related_parties: RelatedParties {
                        debtor: Debtor {
                            name: "John Doe".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "123 Main Street".to_string(),
                                    "City, Country".to_string(),
                                ],
                            },
                        },
                        creditor: Creditor {
                            name: "Jane Smith".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "456 Secondary Street".to_string(),
                                    "Another City, Country".to_string(),
                                ],
                            },
                        },
                    },
                    remittance_information: RemittanceInformation {
                        unstructured: "Payment for invoice #1235".to_string(),
                    },
                },
            ],
        },
        Transaction {
            amount: Amount {
                currency: "EUR".to_string(),
                value: "-150.00".to_string(),
            },
            credit_debit_indicator: "DBIT".to_string(),
            status: "BOOK".to_string(),
            booking_date: EntryDate {
                date: "2023-05-15".to_string(),
            },
            value_date: EntryDate {
                date: "2023-05-15".to_string(),
            },
            account_servicer_reference: "Ref12346".to_string(),
            bank_transaction_code: BankTransactionCode {
                domain: Domain {
                    code: "PMNT".to_string(),
                    family: Family {
                        code: "ICDT".to_string(),
                        sub_family_code: "DMCT".to_string(),
                    },
                },
            },
            entry_details: vec![
                TransactionDetails {
                    references: References {
                        account_servicer_reference: "Ref67892".to_string(),
                    },
                    amount_details: AmountDetails {
                        transaction_amount: Amount {
                            currency: "EUR".to_string(),
                            value: "-50.00".to_string(),
                        },
                    },
                    related_parties: RelatedParties {
                        debtor: Debtor {
                            name: "John Doe".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "123 Main Street".to_string(),
                                    "City, Country".to_string(),
                                ],
                            },
                        },
                        creditor: Creditor {
                            name: "Jane Smith".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "456 Secondary Street".to_string(),
                                    "Another City, Country".to_string(),
                                ],
                            },
                        },
                    },
                    remittance_information: RemittanceInformation {
                        unstructured: "Payment for invoice #1236".to_string(),
                    },
                },
                TransactionDetails {
                    references: References {
                        account_servicer_reference: "Ref67893".to_string(),
                    },
                    amount_details: AmountDetails {
                        transaction_amount: Amount {
                            currency: "EUR".to_string(),
                            value: "-50.00".to_string(),
                        },
                    },
                    related_parties: RelatedParties {
                        debtor: Debtor {
                            name: "John Doe".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "123 Main Street".to_string(),
                                    "City, Country".to_string(),
                                ],
                            },
                        },
                        creditor: Creditor {
                            name: "Jane Smith".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "456 Secondary Street".to_string(),
                                    "Another City, Country".to_string(),
                                ],
                            },
                        },
                    },
                    remittance_information: RemittanceInformation {
                        unstructured: "Payment for invoice #1237".to_string(),
                    },
                },
                TransactionDetails {
                    references: References {
                        account_servicer_reference: "Ref67894".to_string(),
                    },
                    amount_details: AmountDetails {
                        transaction_amount: Amount {
                            currency: "EUR".to_string(),
                            value: "-50.00".to_string(),
                        },
                    },
                    related_parties: RelatedParties {
                        debtor: Debtor {
                            name: "John Doe".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "123 Main Street".to_string(),
                                    "City, Country".to_string(),
                                ],
                            },
                        },
                        creditor: Creditor {
                            name: "Jane Smith".to_string(),
                            postal_address: PostalAddress {
                                address_lines: vec![
                                    "456 Secondary Street".to_string(),
                                    "Another City, Country".to_string(),
                                ],
                            },
                        },
                    },
                    remittance_information: RemittanceInformation {
                        unstructured: "Payment for invoice #1238".to_string(),
                    },
                },
            ],
        },
    ];

    generate_camt053(
        file_path,
        "Message12345",
        "2023-05-16 12:30:00",
        "Statement12345",
        "2023-05-15 10:00:00",
        "DE89370400440532013000",
        "OPBD",
        "1000.00",
        "EUR",
        "CRDT",
        "2023-05-14",
        transactions,
    )
    .expect("Failed to generate CAMT.053 file");

    let file = File::open(file_path).expect("Failed to open generated CAMT.053 file");
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut in_document = false;
    let mut in_group_header = false;
    let mut in_statement = false;
    let mut in_account = false;
    let mut in_balance = false;
    let mut in_entry = false;

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => match name.local_name.as_str() {
                "Document" => in_document = true,
                "GrpHdr" => in_group_header = true,
                "Stmt" => in_statement = true,
                "Acct" => in_account = true,
                "Bal" => in_balance = true,
                "Ntry" => in_entry = true,
                _ => {}
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                "Document" => in_document = false,
                "GrpHdr" => in_group_header = false,
                "Stmt" => in_statement = false,
                "Acct" => in_account = false,
                "Bal" => in_balance = false,
                "Ntry" => in_entry = false,
                _ => {}
            },
            Ok(XmlEvent::Characters(data)) => {
                if in_group_header {
                    match data.as_str() {
                        "Message12345" => assert!(in_group_header),
                        "2023-05-16 12:30:00" => assert!(in_group_header),
                        _ => {}
                    }
                } else if in_statement {
                    match data.as_str() {
                        "Statement12345" => assert!(in_statement),
                        "2023-05-15 10:00:00" => assert!(in_statement),
                        _ => {}
                    }
                } else if in_account {
                    match data.as_str() {
                        "DE89370400440532013000" => assert!(in_account),
                        _ => {}
                    }
                } else if in_balance {
                    match data.as_str() {
                        "OPBD" => assert!(in_balance),
                        "1000.00" => assert!(in_balance),
                        "CRDT" => assert!(in_balance),
                        "2023-05-14" => assert!(in_balance),
                        _ => {}
                    }
                } else if in_entry {
                    match data.as_str() {
                        "-100.00" => assert!(in_entry),
                        "DBIT" => assert!(in_entry),
                        "BOOK" => assert!(in_entry),
                        "2023-05-14" => assert!(in_entry),
                        "Ref12345" => assert!(in_entry),
                        "PMNT" => assert!(in_entry),
                        "ICDT" => assert!(in_entry),
                        "DMCT" => assert!(in_entry),
                        "Ref67890" => assert!(in_entry),
                        "John Doe" => assert!(in_entry),
                        "Jane Smith" => assert!(in_entry),
                        "Payment for invoice #1234" => assert!(in_entry),
                        _ => {}
                    }
                }
            }
            Err(e) => panic!("Error: {}", e),
            _ => {}
        }
    }

//    std::fs::remove_file(file_path).expect("Failed to remove test file");
}
