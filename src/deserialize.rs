use crate::objects::transactions::*;
use serde::de::{Deserialize, Error, Visitor};
use std::fmt;

#[derive(Default)]
pub struct TransactionRowMapVisitor;

impl<'de> Visitor<'de> for TransactionRowMapVisitor {
    type Value = Transaction;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        // Assume ordered CSV for now
        let Some((_, type_tag)) = map.next_entry::<&str, &str>()? else {
            return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
        };

        let Some((_, client_raw)) = map.next_entry::<&str, &str>()? else {
            return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
        };
        // TODO(juf): Minor potential for DRY, we parse+error-handle 3 times here
        // which could be factored out.
        let client = match client_raw.parse() {
            Ok(num) => num,
            Err(err) => {
                return Err(A::Error::custom(format!(
                    "expected integer, but failed to parse {} due to {}",
                    &client_raw, err
                )));
            }
        };
        let Some((_, tx_id_raw)) = map.next_entry::<&str, &str>()? else {
            return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
        };
        let tx_id = match tx_id_raw.parse() {
            Ok(num) => num,
            Err(err) => {
                return Err(A::Error::custom(format!(
                    "expected integer, but failed to parse {} due to {}",
                    &tx_id_raw, err
                )));
            }
        };
        let Some((_, amount_raw)) = map.next_entry::<&str, Option<&str>>()? else {
            return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
        };
        let amount = amount_raw.as_ref().map(|str| str.parse());
        match type_tag {
            DEPOSIT => match amount {
                Some(Ok(dec)) => Ok(Transaction::Deposit(Metadata::new(client, tx_id), dec)),
                Some(Err(err)) => Err(A::Error::custom(format!(
                    "expected decimal, but failed to parse {:?} due to {}",
                    &amount_raw, err
                ))),
                None => Err(A::Error::missing_field(
                    "transaction of type deposit requires an amount",
                )),
            },
            WITHDRAWAL => match amount {
                Some(Ok(dec)) => Ok(Transaction::Withdrawal(Metadata::new(client, tx_id), dec)),
                Some(Err(err)) => Err(A::Error::custom(format!(
                    "expected decimal, but failed to parse {:?} due to {}",
                    &amount_raw, err
                ))),
                None => Err(A::Error::missing_field(
                    "transaction of type withdrawal requires an amount",
                )),
            },
            // For Types without amount we do not care if it exists for now. If it exists we
            // silently drop it (this comment has been written while I have not yet read the
            // whole PDF)
            DISPUTE => Ok(Transaction::Dispute(Metadata::new(client, tx_id))),
            RESOLVE => Ok(Transaction::Resolve(Metadata::new(client, tx_id))),
            CHARGEBACK => Ok(Transaction::Chargeback(Metadata::new(client, tx_id))),
            unknown_variant => Err(A::Error::unknown_variant(unknown_variant, &VALID_VARIANTS)),
        }
        // TODO(juf): Come up with more robust solution which is invariant to the header order
    }
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(TransactionRowMapVisitor)
    }
}

#[cfg(test)]
mod tests {
    use csv::Result;
    use rust_decimal::dec;

    use crate::objects::transactions::{Metadata, Transaction};

    #[test]
    fn empty_row_fails_maybe_not_so_good() {
        let doc = r#"
        type,client,tx,amount

        "#;
        let mut rdr = csv::Reader::from_reader(doc.as_bytes());
        let mut iter = rdr.deserialize();
        let next: Option<Result<Transaction>> = iter.next();
        assert!(next.expect("row should be Some").is_err());
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
        let next: Option<Result<Transaction>> = iter.next();
        assert_eq!(
            next.expect("row should be Some")
                .expect("row should be Transaction"),
            Transaction::Deposit(Metadata::new(2, 3), dec!(4.0))
        );
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
        let next: Option<Result<Transaction>> = iter.next();
        assert_eq!(
            next.expect("row should be Some")
                .expect("row should be Transaction"),
            Transaction::Deposit(Metadata::new(2, 3), dec!(4.0))
        );
        let next: Option<Result<Transaction>> = iter.next();
        assert_eq!(
            next.expect("row should be Some")
                .expect("row should be Transaction"),
            Transaction::Withdrawal(Metadata::new(2, 4), dec!(4.0))
        );
    }
}
