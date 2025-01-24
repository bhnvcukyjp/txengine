#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![warn(clippy::pedantic)]

mod account;
mod manager;
mod transaction;

use csv::Trim;

use account::AccountRecord;
use manager::AccountManager;
use transaction::Transaction;

use std::io::{BufRead, Write};

/// The `process_file` function parses a CSV file containing transaction data and generates
/// a list of clients along with their respective balances.
///
/// # Errors
/// Due to time constraint anyhow crate was used to bubble up two kind of errors:
/// - serialization and deserialization fails from serde
/// - io read and write related fails
///
pub fn process_file<R: BufRead, W: Write>(
    buf_reader: &mut R,
    writer: &mut W,
) -> anyhow::Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(buf_reader);
    let mut raw_record = csv::ByteRecord::new();
    let headers = rdr.byte_headers()?.clone();

    let mut account_manager = AccountManager::default();

    while rdr.read_byte_record(&mut raw_record)? {
        let tx: Transaction = raw_record.deserialize(Some(&headers))?;
        account_manager.process_transaction(&tx);
    }

    let mut csv_wrt = csv::Writer::from_writer(vec![]);
    for (client_id, acc) in account_manager.into_inner() {
        csv_wrt.serialize(AccountRecord::new(client_id, &acc))?;
    }

    writer.write_all(csv_wrt.into_inner()?.as_slice())?;

    Ok(())
}
