use std::collections::HashMap;

use rust_decimal::Decimal;

use crate::objects::{
    accounts::Account,
    transactions::{Metadata, Transaction},
};

pub(crate) struct Processor {
    account_store: HashMap<u16, Account>,
    txn_cache: HashMap<u32, Decimal>,
}

impl Processor {
    pub(crate) fn new() -> Self {
        Self {
            account_store: HashMap::new(),
            txn_cache: HashMap::new(),
        }
    }

    // Assumptions:
    // 1. Locked means no transactions associated with the account are being processed any more
    pub(crate) fn process_one(&mut self, txn: Transaction) {
        let acc_metadata = txn.get_metadata();
        let acc_id = acc_metadata.client;
        let acc = self.create_account_if_not_exists(acc_id);
        if acc.is_locked() {
            // skip any further transactions? The PDF does not really specify what locked means
            return;
        }
        match txn {
            Transaction::Deposit(Metadata { client: _, tx_id }, amount) => {
                if let Some(acc) = self.account_store.get_mut(&acc_id) {
                    acc.deposit(amount);
                    self.txn_cache.entry(tx_id).or_insert(amount);
                }
            }
            Transaction::Withdrawal(Metadata { client: _, tx_id }, amount) => {
                if let Some(acc) = self.account_store.get_mut(&acc_id) {
                    acc.withdraw(amount);
                    self.txn_cache.entry(tx_id).or_insert(amount);
                }
            }
            Transaction::Dispute(Metadata { client: _, tx_id }) => {
                if let Some(amount) = self.txn_cache.get(&tx_id)
                    && let Some(acc) = self.account_store.get_mut(&acc_id)
                {
                    acc.withdraw(*amount);
                };
            }
            Transaction::Resolve(Metadata { client: _, tx_id }) => {
                if let Some(amount) = self.txn_cache.get(&tx_id)
                    && let Some(acc) = self.account_store.get_mut(&acc_id)
                {
                    acc.resolve(*amount);
                };
            }
            Transaction::Chargeback(Metadata { client: _, tx_id }) => {
                if let Some(amount) = self.txn_cache.get(&tx_id)
                    && let Some(acc) = self.account_store.get_mut(&acc_id)
                {
                    acc.chargeback(*amount);
                };
            }
        };
    }

    fn create_account_if_not_exists(&mut self, id: u16) -> &Account {
        self.account_store.entry(id).or_default()
    }
}
