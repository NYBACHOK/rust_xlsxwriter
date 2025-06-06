// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{CustomSerializeField, SerializeFieldOptions, Workbook, XlsxError};
use rust_xlsxwriter_derive::XlsxSerialize;
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet_with_low_memory();

    // Not serialized.
    worksheet.write(0, 0, "column1")?;
    worksheet.write(0, 1, "column2")?;
    worksheet.write(0, 2, "column4")?;

    worksheet.write(1, 0, 1)?;
    worksheet.write(1, 1, 2)?;
    worksheet.write(1, 2, 4)?;
    worksheet.write(2, 0, 1)?;
    worksheet.write(2, 1, 2)?;
    worksheet.write(2, 2, 4)?;
    worksheet.write(3, 0, 1)?;
    worksheet.write(3, 1, 2)?;
    worksheet.write(3, 2, 4)?;
    worksheet.write(4, 0, 1)?;
    worksheet.write(4, 1, 2)?;
    worksheet.write(4, 2, 4)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. Skip fields.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet_with_low_memory();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        column1: u8,
        column2: u8,
        column3: u8,
        column4: u8,
    }

    let data = MyStruct {
        column1: 1,
        column2: 2,
        column3: 3,
        column4: 4,
    };

    let header_options = SerializeFieldOptions::new().set_custom_headers(&[
        CustomSerializeField::new("column1"),
        CustomSerializeField::new("column2"),
        CustomSerializeField::new("column3").skip(true),
        CustomSerializeField::new("column4"),
    ]);

    worksheet.serialize_headers_with_options(0, 0, &data, &header_options)?;

    worksheet.serialize(&data)?;
    worksheet.serialize(&data)?;
    worksheet.serialize(&data)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. Skip fields.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet_with_low_memory();

    // Create a serializable test struct.
    #[derive(Serialize, XlsxSerialize)]
    struct MyStruct {
        column1: u8,
        column2: u8,

        #[xlsx(skip)]
        column3: u8,

        column4: u8,
    }

    let data = MyStruct {
        column1: 1,
        column2: 2,
        column3: 3,
        column4: 4,
    };

    worksheet.set_serialize_headers::<MyStruct>(0, 0)?;

    worksheet.serialize(&data)?;
    worksheet.serialize(&data)?;
    worksheet.serialize(&data)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_serde17_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde17")
        .set_function(create_new_xlsx_file_1)
        .unique("optimize1")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_optimize_serde17_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde17")
        .set_function(create_new_xlsx_file_2)
        .unique("optimize2")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_optimize_serde17_3() {
    let test_runner = common::TestRunner::new()
        .set_name("serde17")
        .set_function(create_new_xlsx_file_3)
        .unique("optimize3")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
