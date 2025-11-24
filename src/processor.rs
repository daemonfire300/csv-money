use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::objects::{
    accounts::Account,
    transactions::{Metadata, Transaction, TransactionState},
};

pub(crate) struct Processor {
    account_store: HashMap<u16, Account>,
    txn_cache: HashMap<u32, (Decimal, TransactionState)>,
}

impl Processor {
    pub(crate) fn new() -> Self {
        Self {
            account_store: HashMap::new(),
            txn_cache: HashMap::new(),
        }
    }

    pub(crate) fn get_account_store_ref(&self) -> &HashMap<u16, Account> {
        &self.account_store
    }

    // Assumptions:
    // 1. Locked means no transactions associated with the account are being processed any more
    pub(crate) fn process_one(&mut self, txn: Transaction) {
        let metadata = txn.get_metadata();
        let acc_id = metadata.client;
        let acc = self.create_account_if_not_exists(acc_id);
        if acc.is_locked() {
            // skip any further transactions? The PDF does not really specify what locked means
            return;
        }
        match txn {
            Transaction::Deposit(Metadata { client: _, tx_id }, amount) => {
                if let Some(acc) = self.account_store.get_mut(&acc_id) {
                    match self.txn_cache.get(&tx_id) {
                        None => {
                            acc.deposit(amount);
                            self.txn_cache
                                .insert(tx_id, (amount, TransactionState::default()));
                        }
                        _ => {} // ignore double reporting of deposit
                    };
                }
            }
            Transaction::Withdrawal(Metadata { client: _, tx_id }, amount) => {
                if let Some(acc) = self.account_store.get_mut(&acc_id) {
                    acc.withdraw(amount);
                    self.txn_cache
                        .entry(tx_id)
                        .or_insert((amount, TransactionState::default()));
                }
            }
            Transaction::Dispute(Metadata { client: _, tx_id }) => {
                if let Some((amount, state)) = self.txn_cache.get_mut(&tx_id)
                    && let Some(acc) = self.account_store.get_mut(&acc_id)
                {
                    match state {
                        TransactionState::Initial => {
                            acc.dispute(*amount);
                            *state = TransactionState::Disputed;
                        }
                        _ => {} // Do nothing. There is not valid transition for this state and
                                // operation type.
                    }
                };
            }
            Transaction::Resolve(Metadata { client: _, tx_id }) => {
                if let Some((amount, state)) = self.txn_cache.get_mut(&tx_id)
                    && let Some(acc) = self.account_store.get_mut(&acc_id)
                {
                    // NOTE(juf): Once a transaction has been Resolved, we could think about
                    // removing it from the cache, _but_ what if we receive the same transaction
                    // again later? We would deposit the amount again.
                    match state {
                        TransactionState::Disputed => {
                            acc.resolve(*amount);
                            *state = TransactionState::Finalized;
                        }
                        _ => {} // Do nothing. There is not valid transition for this state and
                                // operation type.
                    }
                };
            }
            Transaction::Chargeback(Metadata { client: _, tx_id }) => {
                if let Some((amount, state)) = self.txn_cache.get_mut(&tx_id)
                    && let Some(acc) = self.account_store.get_mut(&acc_id)
                {
                    // NOTE(juf): Once a transaction has been charged back, we could think about
                    // removing it from the cache, _but_ what if we receive the same transaction
                    // again later? We would deposit the amount again.
                    match state {
                        TransactionState::Disputed => {
                            acc.chargeback(*amount);
                            *state = TransactionState::Finalized;
                        }
                        _ => {} // Do nothing. There is not valid transition for this state and
                                // operation type.
                    }
                };
            }
        };
    }

    fn create_account_if_not_exists(&mut self, id: u16) -> &Account {
        self.account_store.entry(id).or_insert(Account::new(id))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rust_decimal::dec;

    use crate::{
        egress::default_csv_egress,
        ingest::default_csv_ingest,
        objects::{accounts::Account, transactions::Transaction},
        processor::Processor,
    };

    #[test]
    fn process_simple_file() {
        // TODO(juf): Create test file, maybe write generator function
        let mut ingest = default_csv_ingest(Path::new("tests/sample1.csv"))
            .expect("Can open file and create ingest");
        let mut p = Processor::new();
        let iter = ingest.deserialize();
        let mut count = 0;
        for row in iter {
            let txn: Transaction = row.expect("Should be valid transaction");
            p.process_one(txn);
            count += 1;
        }
        assert_eq!(
            11, count,
            "Did not receive the expected amount of transactions"
        );
        let out_dir = tempfile::tempdir().expect("Could not create tempdir");
        let out_path = out_dir.path().join("out.csv");
        let mut egress = default_csv_egress(&out_path).expect("should get default egress writer");
        let mut count = 0;
        let mut ordered_accounts: Vec<_> = p
            .get_account_store_ref()
            .iter()
            .map(|(_, v)| v)
            .cloned()
            .collect();
        ordered_accounts.sort_by_key(|acc| acc.id);
        for (_, account) in p.get_account_store_ref().iter() {
            egress.serialize(account).expect("can write account row");
            count += 1;
        }
        assert_eq!(
            2, count,
            "Did not receive the expected amount of account statements"
        );
        assert_eq!(
            vec![
                Account {
                    id: 1,
                    locked: false,
                    available: dec!(1.5),
                    held: dec!(0)
                },
                Account {
                    id: 2,
                    locked: false,
                    available: dec!(2.0),
                    held: dec!(0)
                }
            ],
            ordered_accounts
        )
    }

    #[test]
    fn process_dispute_with_and_without_resolve() {
        // TODO(juf): Create test file, maybe write generator function
        let mut ingest = default_csv_ingest(Path::new("tests/sample-dispute-resolve-1.csv"))
            .expect("Can open file and create ingest");
        let mut p = Processor::new();
        let iter = ingest.deserialize();
        let mut count = 0;
        for row in iter {
            let txn: Transaction = row.expect("Should be valid transaction");
            p.process_one(txn);
            count += 1;
        }
        assert_eq!(
            8, count,
            "Did not receive the expected amount of transactions"
        );
        let out_dir = tempfile::tempdir().expect("Could not create tempdir");
        let out_path = out_dir.path().join("out.csv");
        let mut egress = default_csv_egress(&out_path).expect("should get default egress writer");
        let mut count = 0;
        let mut ordered_accounts: Vec<_> = p
            .get_account_store_ref()
            .iter()
            .map(|(_, v)| v)
            .cloned()
            .collect();
        ordered_accounts.sort_by_key(|acc| acc.id);
        for (_, account) in p.get_account_store_ref().iter() {
            egress.serialize(account).expect("can write account row");
            count += 1;
        }
        assert_eq!(
            2, count,
            "Did not receive the expected amount of account statements"
        );
        assert_eq!(
            vec![
                Account {
                    id: 1,
                    locked: false,
                    available: dec!(1.0),
                    held: dec!(2.0)
                },
                Account {
                    id: 2,
                    locked: false,
                    available: dec!(2.0),
                    held: dec!(0)
                }
            ],
            ordered_accounts
        )
    }

    #[test]
    fn process_simple_all_types() {
        // TODO(juf): Create test file, maybe write generator function
        let mut ingest =
            default_csv_ingest(Path::new("tests/sample-3-accounts-all-types-simple-1.csv"))
                .expect("Can open file and create ingest");
        let mut p = Processor::new();
        let iter = ingest.deserialize();
        let mut count = 0;
        for row in iter {
            let txn: Transaction = row.expect("Should be valid transaction");
            p.process_one(txn);
            count += 1;
        }
        assert_eq!(
            26, count,
            "Did not receive the expected amount of transactions"
        );
        let out_dir = tempfile::tempdir().expect("Could not create tempdir");
        let out_path = out_dir.path().join("out.csv");
        let mut egress = default_csv_egress(&out_path).expect("should get default egress writer");
        let mut count = 0;
        let mut ordered_accounts: Vec<_> = p
            .get_account_store_ref()
            .iter()
            .map(|(_, v)| v)
            .cloned()
            .collect();
        ordered_accounts.sort_by_key(|acc| acc.id);
        for (_, account) in p.get_account_store_ref().iter() {
            egress.serialize(account).expect("can write account row");
            count += 1;
        }
        assert_eq!(
            5, count,
            "Did not receive the expected amount of account statements"
        );
        assert_eq!(
            vec![
                Account {
                    id: 1,
                    locked: false,
                    available: dec!(13.3456),
                    held: dec!(2.0)
                },
                Account {
                    id: 2,
                    locked: false,
                    available: dec!(2.0),
                    held: dec!(0.0)
                },
                Account {
                    id: 3,
                    locked: false,
                    available: dec!(11.1001),
                    held: dec!(0)
                },
                Account {
                    id: 4,
                    locked: false,
                    available: dec!(2.0),
                    held: dec!(0)
                },
                Account {
                    id: 10,
                    locked: true,
                    available: dec!(102.24),
                    held: dec!(0.00)
                }
            ],
            ordered_accounts
        )
    }
}
