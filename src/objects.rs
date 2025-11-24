pub(crate) mod transactions {
    use rust_decimal::Decimal;

    // TOOD(juf): Safety improvement, write macro to turn VALID_VARIANTS into consts.
    // This ties the strings to one source of truth. Currently you can still have a missing
    // variant.
    // This does not solve the exhaustiveness of the Deserialize type tag checking.
    // Have to come up with an idea for that later (maybe).
    pub(crate) const DEPOSIT: &str = "deposit";
    pub(crate) const WITHDRAWAL: &str = "withdrawal";
    pub(crate) const DISPUTE: &str = "dispute";
    pub(crate) const RESOLVE: &str = "resolve";
    pub(crate) const CHARGEBACK: &str = "chargeback";
    pub(crate) const VALID_VARIANTS: [&str; 5] =
        [DEPOSIT, WITHDRAWAL, DISPUTE, RESOLVE, CHARGEBACK];

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
    #[derive(Default)]
    pub enum TransactionState {
        #[default]
        Initial,
        Finalized,
        Disputed,
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

    #[derive(Debug, PartialEq, Eq)]
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
            self.available - self.held
        }

        pub(crate) fn dispute(&mut self, amount: Decimal) {
            self.held += amount;
            self.available -= amount;
        }

        pub(crate) fn resolve(&mut self, amount: Decimal) {
            self.held -= amount;
            self.available += amount;
        }

        pub(crate) fn chargeback(&mut self, amount: Decimal) {
            self.held -= amount;
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
