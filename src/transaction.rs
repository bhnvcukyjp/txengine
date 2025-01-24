use rust_decimal::Decimal;
use serde::{de, de::Error, Deserialize};

#[derive(Debug, Deserialize)]
pub struct TxDetails {
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub tx_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct TxDetailsWithAmount {
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    pub tx_id: u32,
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub amount: Decimal,
}

#[derive(Debug)]
pub enum Transaction {
    Deposit(TxDetailsWithAmount),
    Withdrawal(TxDetailsWithAmount),
    Dispute(TxDetails),
    Resolve(TxDetails),
    Chargeback(TxDetails),
}

// credits for custom deserializer: https://github.com/BurntSushi/rust-csv/issues/211
impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_seq(TransactionVisitor)
    }
}

struct TransactionVisitor;

impl<'de> de::Visitor<'de> for TransactionVisitor {
    type Value = Transaction;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Transaction")
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let discrim = seq
            .next_element::<&'de str>()?
            .ok_or_else(|| A::Error::missing_field("discriminant"))?;

        // wrap the remainder of the SeqAccess back into a Deserializer
        let variant = de::value::SeqAccessDeserializer::new(seq);

        match discrim {
            "deposit" => TxDetailsWithAmount::deserialize(variant).map(Transaction::Deposit),
            "withdrawal" => TxDetailsWithAmount::deserialize(variant).map(Transaction::Withdrawal),
            "dispute" => TxDetails::deserialize(variant).map(Transaction::Dispute),
            "chargeback" => TxDetails::deserialize(variant).map(Transaction::Chargeback),
            "resolve" => TxDetails::deserialize(variant).map(Transaction::Resolve),
            x => Err(A::Error::unknown_variant(
                x,
                &["deposit", "withdrawal", "dispute", "chargeback", "resolve"],
            )),
        }
    }
}
