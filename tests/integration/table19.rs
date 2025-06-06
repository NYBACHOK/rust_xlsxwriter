// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, TableColumn, Workbook, XlsxError};

// Write a table with a user specified header. This also tests whitespace in headers.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let columns = vec![
        TableColumn::default(),
        TableColumn::default(),
        TableColumn::default(),
        TableColumn::new().set_header(" Column4 "),
    ];

    let table = Table::new().set_columns(&columns);

    worksheet.add_table(2, 2, 12, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

// Write a table that takes the header name from the worksheet cell data.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_range_width(2, 5, 10.288)?;

    // Write the header string, the table should read this and add it.
    worksheet.write(2, 5, " Column4 ")?;

    let table = Table::new();

    worksheet.add_table(2, 2, 12, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table19_1() {
    let test_runner = common::TestRunner::new()
        .set_name("table19")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_table19_2() {
    let test_runner = common::TestRunner::new()
        .set_name("table19")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
