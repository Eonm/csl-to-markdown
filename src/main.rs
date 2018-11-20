// Name : csl_to_markdown
// Description : Makes your Zotero's/Mendeley's bibliography export compatible with markdown
// Author : eonm<eon.mathis@gmail.com>
// Licence : MIT
// usage : ./csl_to_markdown my_csl_file.csl > my_compatible_csl_file.csl

use std::fs::File;
use std::io::{Read, Cursor, Write};
use std::path::Path;

extern crate regex;
use regex::Regex;

extern crate quick_xml;
use quick_xml::Writer;
use quick_xml::Reader;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("csl_to_markdown")
        .version("0.2.0")
        .author("eonm <eon.mathis@gmail.com>")
        .about("Makes your Zotero's/Mendeley's bibliography export compatible with markdown")
        .arg(Arg::with_name("input")
                  .short("i")
                  .long("input")
                  .required(true)
                  .takes_value(true)
                  .help("input csl file"))
         .arg(Arg::with_name("output")
                  .short("o")
                  .long("output")
                  .required(false)
                  .takes_value(true)
                  .help("output csl file"))
        .get_matches();

    let csl_file_location = matches.value_of("input").expect("Can't get csl file location");
    let output_file = matches.value_of("output");
    let parsed_xml = parse_xml_file(&Path::new(csl_file_location));
    let result_string = String::from_utf8_lossy(&parsed_xml);

    match output_file {
        Some(file_name) => {
            let mut f = File::create(file_name).expect("Failed to create output file");
            f.write_all(&parsed_xml).expect("Failed to write output in the file");
        },
        None => print!("{}", remove_formated_amp(result_string.to_string())),
    };
}


fn parse_xml_string (xml_string : &str) -> Vec<u8> {
     let reader = parse_xml(Reader::from_str(&xml_string));
     reader
}


fn parse_xml_file (xml_file_location : &Path) -> Vec<u8> {
        let reader = match File::open(xml_file_location) {
        Ok(mut xml_file) => {
            let mut buf = Vec::new();
            xml_file.read_to_end(&mut buf);
            buf
        },
        Err(why) => panic!("Failed to open XML file {:?} : {:?}", xml_file_location, why),
    };

    parse_xml(Reader::from_reader(&reader))
}

fn parse_xml (mut reader : Reader<&[u8]>) -> Vec<u8> {
    let mut in_bibliography = false;
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"title" => {
                 let mut elem = BytesStart::owned(b"title".to_vec(), "title".len());
                 elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                 assert!(writer.write_event(Event::Start(elem)).is_ok());
            },
            Ok(Event::End(ref e)) if e.name() == b"title" => {
                assert!(writer.write_event(Event::Text(BytesText::from_plain_str("_MD_"))).is_ok());
                assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"title"))).is_ok());
            },
            Ok(Event::Start(ref e)) if e.name() == b"title-short" => {
                let mut elem = BytesStart::owned(b"title-short".to_vec(), "title-short".len());
                elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                assert!(writer.write_event(Event::Start(elem)).is_ok());
            },
            Ok(Event::End(ref e)) if e.name() == b"title-short" => {
                assert!(writer.write_event(Event::Text(BytesText::from_plain_str("_MD_"))).is_ok());
                assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"title-short"))).is_ok());
            },
            Ok(Event::Start(ref e)) if e.name() == b"id" => {
                let mut elem = BytesStart::owned(b"id".to_vec(), "id".len());
                elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                assert!(writer.write_event(Event::Start(elem)).is_ok());
            },
            Ok(Event::End(ref e)) if e.name() == b"id" => {
                assert!(writer.write_event(Event::Text(BytesText::from_plain_str("/_MD_"))).is_ok());
                assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"id"))).is_ok());
            },
            Ok(Event::Start(ref e)) if e.name() == b"bibliography" => {
                in_bibliography = true;
                let mut elem = BytesStart::owned(b"bibliography".to_vec(), "bibliography".len());
                elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
                assert!(writer.write_event(Event::Start(elem)).is_ok());
            },
            Ok(Event::End(ref e)) if e.name() == b"bibliography" => {
                in_bibliography = false;
                assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"bibliography"))).is_ok());
            },
            Ok(Event::Start(ref e)) if e.name() == b"layout" => {
                let mut elem = BytesStart::owned(b"layout".to_vec(), 4);
                let mut attrs = e.attributes().collect::<Result<Vec<_>,quick_xml::Error>>().expect("Can't retrieve xml attibutes");
                    for attribute in attrs {
                        match attribute.key {
                            b"suffix" => {
                                let mut suffix_previous_value = String::from_utf8_lossy(&attribute.value).to_string().to_owned();
                                if in_bibliography {
                                    suffix_previous_value.push_str("&#10;");
                                }
                                elem.push_attribute(("suffix", suffix_previous_value.as_str()));
                            },
                            _ => {
                                if in_bibliography {
                                    elem.push_attribute(("suffix", "&#10;"));
                                }
                                elem.push_attribute(attribute)
                            },
                        }
                }
                assert!(writer.write_event(Event::Start(elem)).is_ok());
            },
            Ok(Event::Empty(ref e)) if e.name() == b"text" => {
                    let mut attrs = e.attributes().collect::<Result<Vec<_>,quick_xml::Error>>().expect("Can't retrieve xml attibutes");
                    let mut elem = BytesStart::owned(b"text".to_vec(), 4);

                    let mut prefix : Vec<String> = vec!();
                    let mut suffix : Vec<String> = vec!();
                    for attribute in attrs {

                        match attribute.key {
                            b"prefix" => prefix.push(String::from_utf8_lossy(&attribute.value).into_owned()),
                            b"suffix" => suffix.push(String::from_utf8_lossy(&attribute.value).into_owned()),
                            b"font-style" => {
                                if String::from_utf8_lossy(&attribute.value) == "italic" {
                                    prefix.insert(0, "_".to_string());
                                    suffix.push("_".to_string());
                                } else if String::from_utf8_lossy(&attribute.value) == "bold" {
                                    prefix.insert(0, "**".to_string());
                                    suffix.push("**".to_string());
                                };
                                elem.push_attribute(attribute);
                            },
                            _ => elem.push_attribute(attribute),
                        }
                    }
                    if !suffix.is_empty() && !prefix.is_empty() {
                        elem.push_attribute(("prefix", prefix.join("").as_str()));
                        elem.push_attribute(("suffix", suffix.join("").as_str()));
                    };
                    assert!(writer.write_event(Event::Empty(elem)).is_ok());
                },
                Ok(Event::Eof) => break,
                Ok(e) => assert!(writer.write_event(&e).is_ok()),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            }
        buf.clear();
    }

    let result = writer.into_inner().into_inner();
    result
}

fn remove_formated_amp (text : String) -> String {
    let match_escaped_amp = Regex::new(r"&amp;").unwrap();
    let result = match_escaped_amp.replace_all(&text, "&");
    result.to_string()
}

//-------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_should_failed_to_open_unexisting_file() {
        parse_xml_file(Path::new("ce6b20ee7f7797e102f68d15099e7d5b0e8d4c50f98a7865ea168717539ec3aa.csl"));
    }

    #[test]
    fn test_remove_formated_amp() {
        let input_string = "lorem ipsum .&amp; dolor sit amen";
        let expected_string = "lorem ipsum .& dolor sit amen";
        assert_eq!(remove_formated_amp(input_string.to_string()), expected_string);
    }

    #[test]
    fn test_dont_remove_unformated_amp() {
        let input_string = "lorem ipsum .& dolor sit amen";
        let expected_string = "lorem ipsum .& dolor sit amen";
        assert_eq!(remove_formated_amp(input_string.to_string()), expected_string);
    }

    #[test]
    fn test_should_close_xml_tags() {
        let xml_input_string = r#"
            <title>Test title</title>
            <title-short>TT</title-short>
            <id>ce6b20ee7f7797e102f68d15099e7d5b0e8d4c50f98a7865ea168717539ec3aa</id>
            <layout></layout>
            <bibliography></bibliography>
        "#;
        let expected_xml_output_string = r#"
            <title>Test title_MD_</title>
            <title-short>TT_MD_</title-short>
            <id>ce6b20ee7f7797e102f68d15099e7d5b0e8d4c50f98a7865ea168717539ec3aa/_MD_</id>
            <layout></layout>
            <bibliography></bibliography>
        "#;

        let result = parse_xml_string(xml_input_string);
        assert_eq!(result, expected_xml_output_string.as_bytes());
    }

    #[test]
    fn test_should_add_newline_in_bib_layout() {
        let xml_input_string = r#"
            <layout suffix=".">
            </layout>
            <bibliography>
                <layout suffix=".">
                </layout>
            </bibliography>
        "#;
        let expected_xml_output_string = r#"
            <layout suffix=".">
            </layout>
            <bibliography>
                <layout suffix=".&amp;#10;">
                </layout>
            </bibliography>
        "#;

        let result = parse_xml_string(xml_input_string);
        assert_eq!(result, expected_xml_output_string.as_bytes());
    }

    #[test]
    fn test_should_not_add_newline_in_non_bib_layout() {
        let xml_input_string = r#"
            <layout suffix=".">
            </layout>
            <bibliography>
            </bibliography>
        "#;

        let expected_xml_output_string = r#"
            <layout suffix=".">
            </layout>
            <bibliography>
            </bibliography>
        "#;

        let result = parse_xml_string(xml_input_string);
        assert_eq!(result, expected_xml_output_string.as_bytes());
    }

    #[test]
    fn test_should_only_format_italic() {
        let xml_input_string = r#"
            <text/>
            <text font-style="italic"/>
            <text/>
        "#;

        let expected_xml_output_string = r#"
            <text/>
            <text font-style="italic" prefix="_" suffix="_"/>
            <text/>
        "#;

        let result = parse_xml_string(xml_input_string);
        assert_eq!(String::from_utf8_lossy(&result), String::from_utf8_lossy(expected_xml_output_string.as_bytes()));
    }

    #[test]
    fn test_should_only_format_bold() {
        let xml_input_string = r#"
            <text/>
            <text font-style="bold"/>
            <text/>
        "#;

        let expected_xml_output_string = r#"
            <text/>
            <text font-style="bold" prefix="**" suffix="**"/>
            <text/>
        "#;

        let result = parse_xml_string(xml_input_string);
        assert_eq!(result, expected_xml_output_string.as_bytes());
    }

    #[test]
    fn test_should_keep_previous_attributes() {
        let xml_input_string = r#"
            <bibliography id="bibliography"></bibliography>
            <title id="title"></title>
            <title-short id="title-short"></title-short>
            <id id="id"></id>
            <text test-attribute="test" font-style="italic" prefix="test"/>
            <text test-attribute="test" font-style="italic" suffix="test"/>
            <text test-attribute="test" font-style="bold" prefix="test"/>
            <text test-attribute="test" font-style="bold" suffix="test"/>
        "#;
        let expected_xml_output_string = r#"
            <bibliography id="bibliography"></bibliography>
            <title id="title">_MD_</title>
            <title-short id="title-short">_MD_</title-short>
            <id id="id">/_MD_</id>
            <text test-attribute="test" font-style="italic" prefix="_test" suffix="_"/>
            <text test-attribute="test" font-style="italic" prefix="_" suffix="_test"/>
            <text test-attribute="test" font-style="bold" prefix="**test" suffix="**"/>
            <text test-attribute="test" font-style="bold" prefix="**" suffix="**test"/>
        "#;

        let result = parse_xml_string(xml_input_string);
        assert_eq!(String::from_utf8_lossy(&result), String::from_utf8_lossy(expected_xml_output_string.as_bytes()));
    }
}
