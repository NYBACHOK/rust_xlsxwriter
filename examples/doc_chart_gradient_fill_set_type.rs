// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! An example of setting a gradient fill for a chart element with a non-default
//! gradient type.

use rust_xlsxwriter::{
    Chart, ChartGradientFill, ChartGradientFillType, ChartGradientStop, ChartType, Workbook,
    XlsxError,
};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add some data for the chart.
    worksheet.write(0, 0, 10)?;
    worksheet.write(1, 0, 40)?;
    worksheet.write(2, 0, 50)?;
    worksheet.write(3, 0, 20)?;
    worksheet.write(4, 0, 10)?;
    worksheet.write(5, 0, 50)?;

    // Create a new chart.
    let mut chart = Chart::new(ChartType::Column);

    // Add a data series with formatting.
    chart
        .add_series()
        .set_values("Sheet1!$A$1:$A$6")
        .set_format(
            ChartGradientFill::new()
                .set_type(ChartGradientFillType::Rectangular)
                .set_gradient_stops(&[
                    ChartGradientStop::new("#963735", 0),
                    ChartGradientStop::new("#F1DCDB", 100),
                ]),
        );

    // Add the chart to the worksheet.
    worksheet.insert_chart(0, 2, &chart)?;

    // Save the file.
    workbook.save("chart.xlsx")?;

    Ok(())
}
