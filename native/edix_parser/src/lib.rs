// Purpose of this is to force precompilation to include schemas.csv
// schemas.csv is required by transaction.rs
#[rustler::nif]
fn main() {
    include_bytes!("../resources/schemas.csv");
}

use edi::parse;
use rustler::NifStruct;
use std::fs::read_to_string;

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixDocument"]
struct EdixDocument {
    name: String,
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
        name: edi_document.interchanges[0].sender_id.clone().to_string(),
    };
    return Ok(edix_document);
}

rustler::init!("Elixir.Edix.Parser", [main, parse_edi_file]);
