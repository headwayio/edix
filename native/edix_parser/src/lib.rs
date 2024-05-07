#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

use rustler::NifStruct;
use edi::parse;
use std::fs::read_to_string;

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixDocument"]
struct EdixDocument {
    name: String
}

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixParseError"]
struct EdixParseError {
    reason: String,
}

#[rustler::nif]
fn parse_edi_file(input: String) -> Result<EdixDocument, EdixParseError> {
    let edi_string = read_to_string(&input).unwrap();
    let edi_document = parse(&edi_string).unwrap();
    let edix_document = EdixDocument {
        name: edi_document.interchanges[0].sender_id.clone().to_string()
    };
    return Ok(edix_document);
}

rustler::init!("Elixir.Edix.Parser", [add, parse_edi_file]);
