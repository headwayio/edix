use edi::parse as edi_parse;
use rustler::Decoder;
use rustler::NifStruct;
use std::fs::read_to_string;

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixSegment"]
#[rustler(encode)]
struct EdixSegment {
    segment_identifier: String,
    segment_data: Vec<String>,
}

impl<'a> Decoder<'a> for EdixSegment {
    fn decode(term: rustler::Term<'a>) -> Result<Self, rustler::Error> {
        Ok(EdixSegment {
            segment_identifier: term.map_get("segment_identifier")?.decode()?,
            segment_data: term.map_get("segment_data")?.decode()?,
        })
    }
}

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixTransactionSet"]
#[rustler(encode)]
struct EdixTransactionSet {
    transaction_set_control_number: String,
    transaction_set_identifier_code: String,
    transaction_set_date: String,
    transaction_set_time: String,
    transaction_set_name: String,
    transaction_set_segments: Vec<EdixSegment>,
}

impl<'a> Decoder<'a> for EdixTransactionSet {
    fn decode(term: rustler::Term<'a>) -> Result<Self, rustler::Error> {
        Ok(EdixTransactionSet {
            transaction_set_control_number: term
                .map_get("transaction_set_control_number")?
                .decode()?,
            transaction_set_identifier_code: term
                .map_get("transaction_set_identifier_code")?
                .decode()?,
            transaction_set_date: term.map_get("transaction_set_date")?.decode()?,
            transaction_set_time: term.map_get("transaction_set_time")?.decode()?,
            transaction_set_name: term.map_get("transaction_set_name")?.decode()?,
            transaction_set_segments: term.map_get("transaction_set_segments")?.decode()?,
        })
    }
}

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixFunctionalGroup"]
#[rustler(encode)]
struct EdixFunctionalGroup {
    functional_group_control_number: String,
    application_sender_code: String,
    application_receiver_code: String,
    date: String,
    time: String,
    group_reference_number: String,
    transaction_set_control_number: String,
    transaction_sets: Vec<EdixTransactionSet>,
}

impl<'a> Decoder<'a> for EdixFunctionalGroup {
    fn decode(term: rustler::Term<'a>) -> Result<Self, rustler::Error> {
        Ok(EdixFunctionalGroup {
            functional_group_control_number: term
                .map_get("functional_group_control_number")?
                .decode()?,
            application_sender_code: term.map_get("application_sender_code")?.decode()?,
            application_receiver_code: term.map_get("application_receiver_code")?.decode()?,
            date: term.map_get("date")?.decode()?,
            time: term.map_get("time")?.decode()?,
            group_reference_number: term.map_get("group_reference_number")?.decode()?,
            transaction_set_control_number: term
                .map_get("transaction_set_control_number")?
                .decode()?,
            transaction_sets: term.map_get("transaction_sets")?.decode()?,
        })
    }
}

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixInterchangeControl"]
#[rustler(encode)]
struct EdixInterchangeControl {
    interchange_control_number: String,
    sender_id: String,
    receiver_id: String,
    authorization_qualifier: String,
    authorization_information: String,
    security_qualifier: String,
    functional_groups: Vec<EdixFunctionalGroup>,
}

impl<'a> Decoder<'a> for EdixInterchangeControl {
    fn decode(term: rustler::Term<'a>) -> Result<Self, rustler::Error> {
        Ok(EdixInterchangeControl {
            interchange_control_number: term.map_get("interchange_control_number")?.decode()?,
            sender_id: term.map_get("sender_id")?.decode()?,
            receiver_id: term.map_get("receiver_id")?.decode()?,
            authorization_qualifier: term.map_get("authorization_qualifier")?.decode()?,
            authorization_information: term.map_get("authorization_information")?.decode()?,
            security_qualifier: term.map_get("security_qualifier")?.decode()?,
            functional_groups: term.map_get("functional_groups")?.decode()?,
        })
    }
}

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixDocument"]
#[rustler(encode)]
pub struct EdixDocument {
    envelope: Vec<EdixInterchangeControl>,
    segment_delimiter: String,
    sub_element_delimiter: String,
    element_delimiter: String,
}

#[derive(Debug, NifStruct)]
#[module = "Edix.EdixParseError"]
pub struct EdixParseError {
    reason: String,
}

#[rustler::nif]
pub fn parse_file(input: String) -> Result<EdixDocument, EdixParseError> {
    let edi_string = read_to_string(&input).unwrap();
    parse_content(edi_string)
}

#[rustler::nif]
pub fn parse(input: String) -> Result<EdixDocument, EdixParseError> {
    parse_content(input)
}

fn parse_content(input: String) -> Result<EdixDocument, EdixParseError> {
    let edi_document = edi_parse(&input).unwrap();

    let mut interchanges = Vec::new();

    for interchange in edi_document.interchanges {
        let mut functional_groups = Vec::new();

        for functional_group in interchange.functional_groups {
            let mut transaction_sets = Vec::new();

            for transaction_set in functional_group.transactions {
                let mut segments = Vec::new();

                for segment in transaction_set.segments {
                    let mut elements = Vec::new();

                    for element in segment.elements {
                        elements.push(element.to_string());
                    }

                    let edix_segment = EdixSegment {
                        segment_identifier: segment.segment_abbreviation.to_string(),
                        segment_data: elements,
                    };
                    segments.push(edix_segment);
                }

                let edix_transaction_set = EdixTransactionSet {
                    transaction_set_name: transaction_set.transaction_name.to_string(),
                    transaction_set_control_number: transaction_set
                        .transaction_set_control_number
                        .to_string(),
                    transaction_set_identifier_code: transaction_set.transaction_code.to_string(),
                    transaction_set_date: "transaction_set.date.to_string()".to_string(),
                    transaction_set_time: "transaction_set.transaction_set_time.to_string()"
                        .to_string(),
                    transaction_set_segments: segments,
                };

                transaction_sets.push(edix_transaction_set);
            }

            let edix_functional_group = EdixFunctionalGroup {
                functional_group_control_number: functional_group
                    .functional_identifier_code
                    .to_string(),
                application_sender_code: functional_group.application_sender_code.to_string(),
                application_receiver_code: functional_group.application_receiver_code.to_string(),
                date: functional_group.date.to_string(),
                time: functional_group.time.to_string(),
                group_reference_number: functional_group.functional_identifier_code.to_string(),
                transaction_set_control_number: functional_group
                    .functional_identifier_code
                    .to_string(),
                transaction_sets,
            };

            functional_groups.push(edix_functional_group);
        }

        let interchange_control = EdixInterchangeControl {
            interchange_control_number: interchange.interchange_control_number.to_string(),
            sender_id: interchange.sender_id.to_string(),
            receiver_id: interchange.receiver_id.to_string(),
            authorization_qualifier: interchange.authorization_qualifier.to_string(),
            authorization_information: interchange.authorization_information.to_string(),
            security_qualifier: interchange.security_qualifier.to_string(),
            functional_groups,
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

rustler::init!("Elixir.Edix.Parser", [parse, parse_file]);
