macro_rules! assert_processed_input_equals_expected_result {
    (input_file: $input_path:expr, expected_file: $expected_path:expr) => {{
        let file = ::std::fs::File::open($input_path).unwrap();
        let mut buf_reader = ::std::io::BufReader::new(file);

        let mut calculation_buffer = ::std::io::Cursor::new(::std::vec::Vec::new());
        ::engine::process_file(&mut buf_reader, &mut calculation_buffer).unwrap();

        let expected_result = ::std::fs::read($expected_path).unwrap();

        let diff_byte_records = ::csv_diff::csv_diff::CsvByteDiffLocal::new()
            .unwrap()
            .diff(
                ::csv_diff::csv::Csv::with_reader_seek(calculation_buffer.get_ref()),
                ::csv_diff::csv::Csv::with_reader_seek(&expected_result),
            )
            .unwrap();

        assert!(diff_byte_records.as_slice().is_empty());
    }};
}

#[test]
fn it_evaluates_deposits_and_widthdrawals_correctly() {
    assert_processed_input_equals_expected_result!(input_file: "./tests/transactions_1.csv", expected_file: "./tests/result_1.csv");
}

#[test]
fn it_locks_account_after_chargeback() {
    assert_processed_input_equals_expected_result!(input_file: "./tests/transactions_2.csv", expected_file: "./tests/result_2.csv");
}

#[test]
fn it_frees_funds_after_resolution() {
    assert_processed_input_equals_expected_result!(input_file: "./tests/transactions_3.csv", expected_file: "./tests/result_3.csv");
}

#[test]
fn it_ignores_chargeback_on_undisputed_deposits() {
    assert_processed_input_equals_expected_result!(input_file: "./tests/transactions_4.csv", expected_file: "./tests/result_4.csv");
}

#[test]
fn it_removes_disputed_status_of_deposit_after_resolution() {
    assert_processed_input_equals_expected_result!(input_file: "./tests/transactions_5.csv", expected_file: "./tests/result_5.csv");
}

#[test]
fn it_ignores_disputes_when_insufficient_funds() {
    assert_processed_input_equals_expected_result!(input_file: "./tests/transactions_6.csv", expected_file: "./tests/result_6.csv");
}

#[test]
fn it_keeps_up_to_three_decimal_points_of_precision() {
    assert_processed_input_equals_expected_result!(input_file: "./tests/transactions_7.csv", expected_file: "./tests/result_7.csv");
}
