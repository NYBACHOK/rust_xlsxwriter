// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test to demonstrate autofilters.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Write the headers.
    worksheet.write_string(0, 0, "Region")?;
    worksheet.write_string(0, 1, "Item")?;
    worksheet.write_string(0, 2, "Volume")?;
    worksheet.write_string(0, 3, "Month")?;

    // Write the data used in the autofilter.
    let data = common::get_autofilter_data();
    for (row, data) in data.iter().enumerate() {
        let row = 1 + row as u32;
        worksheet.write_string(row, 0, data.0)?;
        worksheet.write_string(row, 1, data.1)?;
        worksheet.write_number(row, 2, data.2)?;
        worksheet.write_string(row, 3, data.3)?;
    }

    worksheet.autofilter(0, 0, 50, 3)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_autofilter01() {
    let test_runner = common::TestRunner::new()
        .set_name("autofilter01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
