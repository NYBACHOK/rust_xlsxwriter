// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{DataValidation, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let data_validation = DataValidation::new().allow_list_strings(&["Foo", "Bar", "Baz"])?;

    worksheet.add_data_validation(1, 2, 1, 2, &data_validation)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_data_validation01() {
    let test_runner = common::TestRunner::new()
        .set_name("data_validation01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
