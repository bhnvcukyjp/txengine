use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug)]
pub struct Funds {
    pub available: Decimal,
    pub held: Decimal,
}

#[derive(Debug)]
pub struct Account {
    pub locked: bool,
    pub funds: Funds,
}

#[derive(Debug)]
pub struct DepositRecord {
    pub amount: Decimal,
    pub disputed: bool,
}

// this struct represents data serialized to a CSV file, where each field maps to a column
#[derive(Debug, Serialize)]
pub struct AccountRecord {
    pub client: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}

impl AccountRecord {
    pub fn new(client_id: u16, acc: &Account) -> Self {
        Self {
            client: client_id,
            available: acc.funds.available,
            held: acc.funds.held,
            total: acc.funds.available + acc.funds.held,
            locked: acc.locked,
        }
    }
}
