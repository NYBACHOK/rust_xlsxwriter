// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! The following example demonstrates skipping fields during serialization by
//! explicitly skipping them via custom headers.

use rust_xlsxwriter::{CustomSerializeField, SerializeFieldOptions, Workbook, XlsxError};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Create a serializable struct.
    #[derive(Deserialize, Serialize)]
    struct Produce {
        fruit: &'static str,
        cost: f64,
        in_stock: bool,
    }

    // Create some data instances.
    let item1 = Produce {
        fruit: "Peach",
        cost: 1.05,
        in_stock: true,
    };

    let item2 = Produce {
        fruit: "Plum",
        cost: 0.15,
        in_stock: true,
    };

    let item3 = Produce {
        fruit: "Pear",
        cost: 0.75,
        in_stock: false,
    };

    // We only need to set a custom header for the field we want to skip.
    let header_options = SerializeFieldOptions::new()
        .set_custom_headers(&[CustomSerializeField::new("in_stock").skip(true)]);

    // Set the serialization location and custom headers.
    worksheet.deserialize_headers_with_options::<Produce>(0, 0, &header_options)?;

    // Serialize the data.
    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;

    // Save the file.
    workbook.save("serialize.xlsx")?;

    Ok(())
}
