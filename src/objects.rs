pub(crate) mod transactions {
    use rust_decimal::{Decimal, dec};
    use serde::de::{
        Deserialize, Deserializer, Error, Expected, MapAccess, Unexpected, VariantAccess, Visitor,
    };
    use std::fmt;

    // TOOD(juf): Safety improvement, write macro to turn VALID_VARIANTS into consts.
    // This ties the strings to one source of truth. Currently you can still have a missing
    // variant.
    // This does not solve the exhaustiveness of the Deserialize type tag checking.
    // Have to come up with an idea for that later (maybe).
    const DEPOSIT: &'static str = "deposit";
    const WITHDRAWAL: &'static str = "withdrawal";
    const DISPUTE: &'static str = "dispute";
    const RESOLVE: &'static str = "resolve";
    const CHARGEBACK: &'static str = "chargeback";
    const VALID_VARIANTS: [&'static str; 5] = [DEPOSIT, WITHDRAWAL, DISPUTE, RESOLVE, CHARGEBACK];

    // TODO(juf): Think about making this more "safe" using the type-system.
    // Currently this allows representing invalid domain entities, e.g.,
    // The program can hold/process/produce a Entry of type Chargeback with a not `None` amount.
    // Good type/api/program design should make this impossible
    pub enum Transaction {
        Deposit(Metadata, Decimal), // LLM use here, see LLM file llm-ref[1], not very helpful
        // answer. Decided to just go read serde docs again: https://serde.rs/deserialize-map.html
        Withdrawal(Metadata, Decimal),
        Dispute(Metadata),
        Resolve(Metadata),
        Chargeback(Metadata),
    }
    pub struct Metadata {
        pub client: u16,
        pub tx_id: u32,
    }

    impl Metadata {
        fn new(client: u16, tx_id: u32) -> Self {
            Metadata { client, tx_id }
        }
    }

    mod deserialize {
        use super::*;
        use rust_decimal::{Decimal, dec};
        use serde::de::{
            Deserialize, Deserializer, Error, Expected, MapAccess, Unexpected, VariantAccess,
            Visitor,
        };
        use std::fmt;
        pub struct TransactionRowMapVisitor;
        impl<'de> Visitor<'de> for TransactionRowMapVisitor {
            type Value = Transaction;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a very special map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                // Assume ordered CSV for now
                let Some((_, type_tag)) = map.next_entry::<&str, &str>()? else {
                    return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
                };

                let Some((_, client)) = map.next_entry::<&str, u16>()? else {
                    return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
                };
                let Some((_, tx_id)) = map.next_entry::<&str, u32>()? else {
                    return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
                };
                let Some((_, amount)) = map.next_entry::<&str, Option<Decimal>>()? else {
                    return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
                };
                match type_tag {
                    DEPOSIT => match amount {
                        Some(dec) => Ok(Transaction::Deposit(Metadata::new(client, tx_id), dec)),
                        None => Err(A::Error::missing_field(
                            "transaction of type deposit requires an amount",
                        )),
                    },
                    WITHDRAWAL => match amount {
                        Some(dec) => Ok(Transaction::Withdrawal(Metadata::new(client, tx_id), dec)),
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
                    unknown_variant => {
                        Err(A::Error::unknown_variant(unknown_variant, &VALID_VARIANTS))
                    }
                }
                // TODO(juf): Come up with more robust solution which is invariant to the header order
            }
        }

        impl<'de> Deserialize<'de> for Transaction {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                todo!()
            }
        }
    }

    pub struct RawEntry {
        pub r#type: String,
        pub client: u16,
        pub tx_id: u32,
        pub amount: Option<Decimal>,
    }
}
