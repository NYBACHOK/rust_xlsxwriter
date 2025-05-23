// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Color, Format, FormatUnderline, Workbook, XlsxError};

// Test to demonstrate simple hyperlinks.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let format = Format::new()
        .set_font_color(Color::Red)
        .set_underline(FormatUnderline::Single);

    worksheet.write_url(0, 0, "http://www.perl.org/")?;
    worksheet.write_url_with_options(1, 0, "http://www.perl.com/", "", "", Some(&format))?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_hyperlink29() {
    let test_runner = common::TestRunner::new()
        .set_name("hyperlink29")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
