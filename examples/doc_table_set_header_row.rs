// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! Example of turning off the default header on a worksheet table.

use rust_xlsxwriter::{Table, Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {
    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Some sample data for the table.
    let items = ["Apples", "Pears", "Bananas", "Oranges"];
    let data = [
        [10000, 5000, 8000, 6000],
        [2000, 3000, 4000, 5000],
        [6000, 6000, 6500, 6000],
        [500, 300, 200, 700],
    ];

    // Write the table data.
    worksheet.write_column(2, 1, items)?;
    worksheet.write_row_matrix(2, 2, data)?;

    // Set the column widths for clarity.
    worksheet.set_column_range_width(1, 6, 12)?;

    // Create a new table and configure the header.
    let table = Table::new().set_header_row(false);

    // Add the table to the worksheet.
    worksheet.add_table(2, 1, 5, 5, &table)?;

    // Save the file to disk.
    workbook.save("tables.xlsx")?;

    Ok(())
}
