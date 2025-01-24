use nohash::IntMap;
use rust_decimal_macros::dec;

use crate::account::{Account, DepositRecord, Funds};
use crate::transaction::{Transaction, TxDetails, TxDetailsWithAmount};

#[derive(Debug, Default)]
pub struct AccountManager {
    accounts: IntMap<u16, Account>,
    deposits: IntMap<u32, DepositRecord>,
}

impl AccountManager {
    pub fn into_inner(self) -> IntMap<u16, Account> {
        self.accounts
    }

    pub fn process_transaction(&mut self, transaction: &Transaction) {
        match transaction {
            Transaction::Deposit(TxDetailsWithAmount {
                amount,
                client_id,
                tx_id,
            }) => {
                // upsert an account with the deposited amount
                match self.accounts.get_mut(client_id) {
                    Some(a) => {
                        a.funds.available += amount;
                    }
                    None => {
                        self.accounts.insert(
                            *client_id,
                            Account {
                                locked: false,
                                funds: Funds {
                                    available: *amount,
                                    held: dec!(0.0),
                                },
                            },
                        );
                    }
                }

                // save the deposit record for possible disputes
                self.deposits.insert(
                    *tx_id,
                    DepositRecord {
                        amount: *amount,
                        disputed: false,
                    },
                );
            }
            Transaction::Withdrawal(TxDetailsWithAmount {
                amount, client_id, ..
            }) => {
                let Some(acc) = self.accounts.get_mut(client_id) else {
                    return;
                };
                let available_after_withdrawal = acc.funds.available - amount;

                // allow withdrawal only when there is enough available funds
                if available_after_withdrawal.is_sign_positive() {
                    acc.funds.available = available_after_withdrawal;
                }
            }
            Transaction::Dispute(TxDetails { tx_id, client_id }) => {
                let Some(deposit) = self.deposits.get_mut(tx_id) else {
                    return;
                };
                let Some(acc) = self.accounts.get_mut(client_id) else {
                    return;
                };
                // ignore the dispute if there are insufficient funds on the account
                // i.e. already withdrawn
                let available_after_dispute = acc.funds.available - deposit.amount;
                if available_after_dispute.is_sign_negative() {
                    return;
                }

                // mark the specified deposit as disputed
                deposit.disputed = true;

                acc.funds.available -= deposit.amount;
                acc.funds.held += deposit.amount;
            }
            Transaction::Chargeback(TxDetails { tx_id, client_id }) => {
                let Some(deposit) = self.deposits.get(tx_id) else {
                    return;
                };
                let Some(acc) = self.accounts.get_mut(client_id) else {
                    return;
                };
                // ignore the chargeback if the transaction is not disputed
                if !deposit.disputed {
                    return;
                }

                acc.funds.held -= deposit.amount;
                acc.locked = true;
            }
            Transaction::Resolve(TxDetails { tx_id, client_id }) => {
                let Some(deposit) = self.deposits.get_mut(tx_id) else {
                    return;
                };
                let Some(acc) = self.accounts.get_mut(client_id) else {
                    return;
                };
                // remove disputed status of the deposit
                deposit.disputed = false;

                acc.funds.held -= deposit.amount;
                acc.funds.available += deposit.amount;
            }
        }
    }
}
