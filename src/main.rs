extern crate scraper;
extern crate serde;
extern crate serde_json;

use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::json;
use std::io::{self, Read};

#[derive(Serialize, PartialEq, Debug)]
pub struct Row {
    values: Vec<String>,
}

// Update the main function to use the new parse_html_table function
fn main() {
    // Read HTML from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");
    println!("input: {}", input);
    // Parse HTML and extract headers and rows
    let (headers, rows) = parse_html_table(&input);

    // Generate JSON
    let json_output = generate_json(headers, rows);
    println!(
        "{}",
        serde_json::to_string_pretty(&json_output).expect("Failed to generate JSON")
    );
}

pub fn parse_html_table(input: &str) -> (Vec<String>, Vec<Row>) {
    let document = Html::parse_document(input);

    let header_selector =
        Selector::parse("table thead tr th").expect("Failed to create header selector");
    let headers: Vec<String> = document
        .select(&header_selector)
        .map(|header| header.text().collect::<String>())
        .collect();

    let row_selector = Selector::parse("table tbody tr").expect("Failed to create row selector");
    let cell_selector = Selector::parse("td").expect("Failed to create cell selector");
    let rows: Vec<Row> = document
        .select(&row_selector)
        .map(|row| {
            let values: Vec<String> = row
                .select(&cell_selector)
                .map(|cell| cell.text().collect::<String>().trim().to_string())
                .collect();
            Row { values }
        })
        .collect();

    (headers, rows)
}

pub fn generate_json(headers: Vec<String>, rows: Vec<Row>) -> serde_json::Value {
    let mut json_rows = Vec::new();
    for row in rows {
        let mut json_row = serde_json::Map::new();
        for (header, value) in headers.iter().zip(row.values.into_iter()) {
            json_row.insert(header.clone(), json!(value));
        }
        json_rows.push(json!(json_row));
    }
    json!(json_rows)
}

// Add the tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_table() {
        let input = r#"
        <table>
            <thead>
                <tr>
                    <th>Name</th>
                    <th>Age</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>Alice</td>
                    <td>30</td>
                </tr>
                <tr>
                    <td>Bob</td>
                    <td>25</td>
                </tr>
            </tbody>
        </table>
        "#;

        let expected_headers = vec!["Name".to_string(), "Age".to_string()];
        let expected_rows = vec![
            Row {
                values: vec!["Alice".to_string(), "30".to_string()],
            },
            Row {
                values: vec!["Bob".to_string(), "25".to_string()],
            },
        ];

        let (headers, rows) = parse_html_table(&input);

        assert_eq!(headers, expected_headers);
        assert_eq!(rows, expected_rows);
    }
    #[test]
    fn test_generate_json() {
        let headers = vec!["Name".to_string(), "Age".to_string()];
        let rows = vec![
            Row {
                values: vec!["Alice".to_string(), "30".to_string()],
            },
            Row {
                values: vec!["Bob".to_string(), "25".to_string()],
            },
        ];

        let expected_output = json!(
            [
                {
                    "Name": "Alice",
                    "Age": "30",
                },
                {
                    "Name": "Bob",
                    "Age": "25",
                }
            ]
        );

        let json_output = generate_json(headers, rows);

        assert_eq!(json_output, expected_output);
    }
}
