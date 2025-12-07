#[cfg(test)]
mod tests {
    use csv::Result;
    use rust_decimal::dec;

    use crate::objects::transactions::{Metadata, Row, Transaction, TxType};

    #[test]
    fn empty_row_fails_maybe_not_so_good() {
        let doc = r#"
        type,client,tx,amount

        "#;
        let mut rdr = csv::Reader::from_reader(doc.as_bytes());
        let mut iter = rdr.deserialize();
        let next: Option<Result<Row>> = iter.next();
        assert!(next.expect("row should be Some").is_err());
    }

    #[test]
    fn single_row_alternative() {
        let doc = r#"type,client,tx,amount
deposit, 2, 3, 4.0"#;
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(doc.as_bytes());
        let headers = rdr.headers().expect("should have headers");
        println!("{:?}", headers);
        let mut iter = rdr.deserialize();
        let next: Option<Result<Row>> = iter.next();
        assert_eq!(
            next.expect("row should be Some")
                .expect("row should be Transaction"),
            Row {
                r#type: TxType::Deposit,
                client: 2,
                tx: 3,
                amount: Some(dec!(4.0))
            }
        );
    }

    #[test]
    fn single_row_alternative_full_transaction() {
        let doc = r#"type,client,tx,amount
deposit, 2, 3, 4.0"#;
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(doc.as_bytes());
        let headers = rdr.headers().expect("should have headers");
        println!("{:?}", headers);
        let mut iter = rdr.deserialize();
        let next: Option<Result<Row>> = iter.next();
        let next = next
            .expect("row should be Some")
            .expect("row should be Transaction");
        assert_eq!(
            next,
            Row {
                r#type: TxType::Deposit,
                client: 2,
                tx: 3,
                amount: Some(dec!(4.0))
            }
        );
        let txn: Transaction = next
            .try_into()
            .expect("row should be convertable into transaction");
        assert_eq!(txn, Transaction::Deposit(Metadata::new(2, 3), dec!(4.0)));
    }

    #[test]
    fn single_row() {
        let doc = r#"type,client,tx,amount
deposit, 2, 3, 4.0"#;
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(doc.as_bytes());
        let headers = rdr.headers().expect("should have headers");
        println!("{:?}", headers);
        let mut iter = rdr.deserialize();
        let next: Option<Result<Row>> = iter.next();
        let next: Transaction = next
            .expect("row should be Some")
            .expect("row should contain no errors")
            .try_into()
            .expect("row should be valid Transaction");
        assert_eq!(next, Transaction::Deposit(Metadata::new(2, 3), dec!(4.0)));
    }

    #[test]
    fn multiple_rows() {
        let doc = r#"type,client,tx,amount
deposit, 2, 3, 4.0
withdrawal, 2, 4, 4.0"#;
        let mut rdr = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .from_reader(doc.as_bytes());
        let headers = rdr.headers().expect("should have headers");
        println!("{:?}", headers);
        let mut iter = rdr.deserialize();
        let next: Option<Result<Row>> = iter.next();
        let next: Transaction = next
            .expect("row should be Some")
            .expect("row should contain no errors")
            .try_into()
            .expect("row should be valid Transaction");
        assert_eq!(next, Transaction::Deposit(Metadata::new(2, 3), dec!(4.0)));
        let next: Option<Result<Row>> = iter.next();
        let next: Transaction = next
            .expect("row should be Some")
            .expect("row should contain no errors")
            .try_into()
            .expect("row should be valid Transaction");
        assert_eq!(
            next,
            Transaction::Withdrawal(Metadata::new(2, 4), dec!(4.0))
        );
    }
}
