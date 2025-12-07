pub(crate) mod transactions {
    use rust_decimal::Decimal;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    pub(crate) enum TxType {
        Deposit,
        Withdrawal,
        Dispute,
        Resolve,
        Chargeback,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    pub(crate) struct Row {
        pub r#type: TxType,
        pub client: u16,
        pub tx: u32,
        pub amount: Option<Decimal>,
    }

    impl From<Row> for Metadata {
        fn from(value: Row) -> Self {
            Metadata {
                client: value.client,
                tx_id: value.tx,
            }
        }
    }

    impl TryFrom<Row> for Transaction {
        type Error = crate::error::Error;

        fn try_from(value: Row) -> Result<Self, Self::Error> {
            match value.r#type {
                TxType::Deposit => {
                    if let Some(amount) = value.amount {
                        Ok(Transaction::Deposit(value.into(), amount))
                    } else {
                        Err(crate::error::Error::InvalidRow(
                            "deposits require an amount".into(),
                        ))
                    }
                }
                TxType::Withdrawal => {
                    if let Some(amount) = value.amount {
                        Ok(Transaction::Withdrawal(value.into(), amount))
                    } else {
                        Err(crate::error::Error::InvalidRow(
                            "withdrawals require an amount".into(),
                        ))
                    }
                }
                TxType::Dispute => Ok(Transaction::Dispute(value.into())),
                TxType::Chargeback => Ok(Transaction::Chargeback(value.into())),
                TxType::Resolve => Ok(Transaction::Resolve(value.into())),
            }
        }
    }

    // TODO(juf): Think about making this more "safe" using the type-system.
    // Currently this allows representing invalid domain entities, e.g.,
    // The program can hold/process/produce a Entry of type Chargeback with a not `None` amount.
    // Good type/api/program design should make this impossible
    #[derive(Debug, PartialEq, Eq)]
    pub enum Transaction {
        Deposit(Metadata, Decimal), // LLM use here, see LLM file llm-ref[1], not very helpful
        // answer. Decided to just go read serde docs again: https://serde.rs/deserialize-map.html
        Withdrawal(Metadata, Decimal),
        Dispute(Metadata),
        Resolve(Metadata),
        Chargeback(Metadata),
    }

    /// TransactionState describes whether a Transaction
    /// has been neither disputed/resolved/chargedback if it's `Initial`.
    /// Been resolved or chargedback if it's `Finalized`.
    /// Is under dispute `Disputed`.
    pub enum TransactionState {
        Initial(InitialState),
        Finalized,
        Disputed(InitialState),
    }

    pub enum InitialState {
        Deposit,
        Withdrawal,
    }

    impl Transaction {
        pub(crate) fn get_metadata(&self) -> &Metadata {
            match self {
                Transaction::Deposit(m, _) => m,
                Transaction::Withdrawal(m, _) => m,
                Transaction::Dispute(m) => m,
                Transaction::Resolve(m) => m,
                Transaction::Chargeback(m) => m,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Metadata {
        pub client: u16,
        pub tx_id: u32,
    }

    impl Metadata {
        pub(crate) fn new(client: u16, tx_id: u32) -> Self {
            Metadata { client, tx_id }
        }
    }
}

pub(crate) mod accounts {
    use rust_decimal::Decimal;

    // println!("Account Size {}", size_of::<Account>());
    // >> 36
    #[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
    pub(crate) struct Account {
        pub(crate) id: u16,            // 16 bit                        |    2 bytes
        pub(crate) locked: bool,       // 1 bit                         |    1 byte
        pub(crate) available: Decimal, // 2x 64bit                      |   16 bytes
        pub(crate) held: Decimal,      // 2x 64bit                      |   16 bytes
                                       // padding                       |    1 byte
    }

    impl Account {
        pub(crate) fn new(id: u16) -> Self {
            Self {
                id,
                ..Default::default()
            }
        }

        pub(crate) fn is_locked(&self) -> bool {
            self.locked
        }

        pub(crate) fn total(&self) -> Decimal {
            // TODO(juf): Check overflow behaviour of Decimal library
            self.available + self.held
        }

        pub(crate) fn dispute(&mut self, amount: Decimal) {
            self.held += amount;
            self.available -= amount;
        }

        pub(crate) fn dispute_withdrawal(&mut self, amount: Decimal) {
            self.held += amount;
        }

        pub(crate) fn resolve(&mut self, amount: Decimal) {
            self.held -= amount;
            self.available += amount;
        }

        pub(crate) fn resolve_withdrawal(&mut self, amount: Decimal) {
            self.held -= amount;
        }

        pub(crate) fn chargeback(&mut self, amount: Decimal) {
            self.held -= amount;
            self.locked = true;
        }

        pub(crate) fn chargeback_withdrawal(&mut self, amount: Decimal) {
            // NOTE(juf): Chargeback of withdrawal is not really clear to me
            // This might have some logical insonsistencies or even bugs in it.
            // I noticed this way too late and weaved it in post-hoc.
            // Technically we also "release" the held funds, so it should be same.
            // But we should credit it back to the account, since it was withdrawn from it.
            self.held -= amount;
            self.available += amount;
            self.locked = true;
        }

        pub(crate) fn deposit(&mut self, amount: Decimal) {
            self.available += amount;
        }

        pub(crate) fn withdraw(&mut self, amount: Decimal) {
            if self.available >= amount {
                self.available -= amount
            }
        }
    }
}
