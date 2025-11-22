pub(crate) mod transactions {
    use rust_decimal::{Decimal, dec};
    use serde::de::{
        Deserialize, Deserializer, Error, Expected, MapAccess, Unexpected, VariantAccess, Visitor,
    };
    use std::fmt;

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
            let Some(type_tag) = map.next_entry::<String, String>()? else {
                return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
            };

            let Some(client) = map.next_entry::<String, u16>()? else {
                return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
            };
            let Some(tx) = map.next_entry::<String, u32>()? else {
                return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
            };
            let Some(amount) = map.next_entry::<String, Option<Decimal>>()? else {
                return Err(A::Error::invalid_length(0, &"expected row of 4 entries"));
            };
            // TODO(juf): Come up with more robust solution which is invariant to the header order
            Ok(Transaction::Deposit(
                Metadata {
                    client: 1,
                    tx_id: 1,
                },
                dec!(12.7123),
            ))
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

    pub struct RawEntry {
        pub r#type: String,
        pub client: u16,
        pub tx_id: u32,
        pub amount: Option<Decimal>,
    }
}
