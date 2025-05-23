// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let format = Format::default();

    worksheet.write_url_with_format(0, 0, "http://www.perl.com/", &format)?;

    worksheet.write_formula_with_format(0, 0, "=1+1", &format)?;
    worksheet.set_formula_result(0, 0, "2");

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_hyperlink19() {
    let test_runner = common::TestRunner::new()
        .set_name("hyperlink19")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
