// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Chart, ChartType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    // Add some test data for the chart(s).
    let data1 = ["A", "B", "C", "D", "E"];
    let data2 = [1, 2, 3, 2, 1];

    worksheet.write_column(0, 0, data1)?;
    worksheet.write_column(0, 1, data2)?;

    let mut chart = Chart::new(ChartType::Column);
    chart.set_axis_ids(45686144, 45722240);
    chart
        .add_series()
        .set_categories("=Sheet1!$A$1:$A$5")
        .set_values("=Sheet1!$B$1:$B$5");

    worksheet.insert_chart(8, 4, &chart)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_chart_column10() {
    let test_runner = common::TestRunner::new()
        .set_name("chart_column10")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
