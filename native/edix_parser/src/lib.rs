use edi::parse;
use rustler::Decoder;
use rustler::NifStruct;
use std::fs::read_to_string;

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixInterchangeControl"]
#[rustler(encode)]
struct EdixInterchangeControl {
    authorization_qualifier: String,
    authorization_information: String,
    security_qualifier: String,
}

impl<'a> Decoder<'a> for EdixInterchangeControl {
    fn decode(term: rustler::Term<'a>) -> Result<Self, rustler::Error> {
        Ok(EdixInterchangeControl {
            authorization_qualifier: term.map_get("authorization_qualifier")?.decode()?,
            authorization_information: term.map_get("authorization_information")?.decode()?,
            security_qualifier: term.map_get("security_qualifier")?.decode()?,
        })
    }
}

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixDocument"]
#[rustler(encode)]
struct EdixDocument {
    envelope: Vec<EdixInterchangeControl>,
    segment_delimiter: String,
    sub_element_delimiter: String,
    element_delimiter: String,
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

    let mut interchanges = Vec::new();

    for interchange in edi_document.interchanges {
        let interchange_control = EdixInterchangeControl {
            authorization_qualifier: interchange.authorization_qualifier.to_string(),
            authorization_information: interchange.authorization_information.to_string(),
            security_qualifier: interchange.security_qualifier.to_string(),
        };
        interchanges.push(interchange_control);
    }

    let edix_document = EdixDocument {
        envelope: interchanges,
        segment_delimiter: edi_document.segment_delimiter.to_string(),
        sub_element_delimiter: edi_document.sub_element_delimiter.to_string(),
        element_delimiter: edi_document.element_delimiter.to_string(),
    };
    Ok(edix_document)
}

rustler::init!("Elixir.Edix.Parser", [parse_edi_file]);
