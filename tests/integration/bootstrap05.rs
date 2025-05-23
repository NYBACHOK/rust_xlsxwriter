// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test case to demonstrate creating a basic file with some numeric cell data.
// This tests also verifies the row span ranges.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_number(0, 0, 1.0)?;
    worksheet.write_number(1, 1, 2.0)?;
    worksheet.write_number(2, 2, 3.0)?;

    worksheet.write_number(0, 4, 1.0)?;
    worksheet.write_number(1, 5, 2.0)?;
    worksheet.write_number(2, 6, 3.0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap05_test_spans() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap05")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
