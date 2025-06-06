// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Table, TableColumn, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_checkbox();

    worksheet.write(0, 0, "Col1")?;
    worksheet.write(1, 0, 1)?;
    worksheet.write(2, 0, 2)?;
    worksheet.write(3, 0, 3)?;
    worksheet.write(4, 0, 4)?;

    worksheet.write(0, 1, "Col2")?;
    worksheet.insert_checkbox(1, 1, true)?;
    worksheet.insert_checkbox(2, 1, false)?;
    worksheet.insert_checkbox(3, 1, false)?;
    worksheet.insert_checkbox(4, 1, true)?;

    // Create a new table and configure it.
    let columns = vec![
        TableColumn::default(),
        TableColumn::new().set_format(&format),
    ];

    let table = Table::new().set_columns(&columns);

    worksheet.add_table(0, 0, 4, 1, &table)?;

    workbook.save(filename)?;

    Ok(())
}

// Test with standard boolean value and format.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format = Format::new().set_checkbox();

    worksheet.write(0, 0, "Col1")?;
    worksheet.write(1, 0, 1)?;
    worksheet.write(2, 0, 2)?;
    worksheet.write(3, 0, 3)?;
    worksheet.write(4, 0, 4)?;

    worksheet.write(0, 1, "Col2")?;
    worksheet.write_boolean(1, 1, true)?;
    worksheet.write_boolean(2, 1, false)?;
    worksheet.write_boolean(3, 1, false)?;
    worksheet.write_boolean(4, 1, true)?;

    // Create a new table and configure it.
    let columns = vec![
        TableColumn::default(),
        TableColumn::new().set_format(&format),
    ];

    let table = Table::new().set_columns(&columns);

    worksheet.add_table(0, 0, 4, 1, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_checkbox07_1() {
    let test_runner = common::TestRunner::new()
        .set_name("checkbox07")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .ignore_file("xl/styles.xml")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_checkbox07_2() {
    let test_runner = common::TestRunner::new()
        .set_name("checkbox07")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
