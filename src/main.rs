use csv::{ReaderBuilder, WriterBuilder};
use rfd::FileDialog;
use std::error::Error;
use regex::Regex;

/// Normalize a string to contain only numbers
fn normalize_numbers(value: &str) -> String {
    let re = Regex::new(r"[^0-9]").unwrap();
    re.replace_all(value, "").to_string()
}

/// Normalize date fields to MM/DD/YYYY format
fn normalize_date(value: &str) -> String {
    match chrono::NaiveDate::parse_from_str(value, "%m/%d/%Y")
        .or_else(|_| chrono::NaiveDate::parse_from_str(value, "%d-%m-%Y"))
    {
        Ok(date) => date.format("%m/%d/%Y").to_string(),
        Err(_) => value.to_string(), // Return as-is if parsing fails
    }
}

/// Process CSV file for normalization
fn process_csv(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the input CSV file
    let input_file = std::fs::File::open(input_path)?;
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(input_file);

    // Create the output CSV file
    let output_file = std::fs::File::create(output_path)?;
    let mut writer = WriterBuilder::new().flexible(true).from_writer(output_file);

    // Write headers first
    if let Some(headers) = reader.headers().ok() {
        writer.write_record(headers)?;
    }

    // Process each record
    for result in reader.records() {
        let record = result?;
        let normalized_record: Vec<String> = record
            .iter()
            .enumerate()
            .map(|(i, field)| {
                // Remove trailing spaces
                let trimmed = field.trim();

                match i {
                    3 => normalize_numbers(trimmed),         // Phone number
                    5 => normalize_numbers(trimmed),         // SSN
                    17 => normalize_numbers(trimmed),        // Postal code
                    18 => normalize_numbers(trimmed),        // Monthly rent
                    19 => normalize_numbers(trimmed),        // Outstanding balance
                    4 | 6 | 7 | 8 | 9 => normalize_date(trimmed), // Date fields
                    _ => trimmed.to_string(),                // Leave other fields unchanged
                }
            })
            .collect();

        writer.write_record(&normalized_record)?;
    }

    writer.flush()?;
    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    // Show file dialog for input CSV
    let input_file = FileDialog::new()
        .add_filter("CSV files", &["csv"])
        .pick_file();

    if input_file.is_none() {
        eprintln!("No file selected. Exiting.");
        return Ok(());
    }
    let input_path = input_file.unwrap();

    // Show file dialog for output CSV
    let output_file = FileDialog::new()
        .set_directory(input_path.parent().unwrap_or_else(|| std::path::Path::new(".")))
        .set_file_name("normalized_output.csv")
        .save_file();

    if output_file.is_none() {
        eprintln!("No output file selected. Exiting.");
        return Ok(());
    }
    let output_path = output_file.unwrap();

    process_csv(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
    )?;

    println!(
        "Normalization complete. Output saved to {}",
        output_path.display()
    );
    Ok(())
}
