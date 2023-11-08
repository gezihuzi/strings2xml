use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::Path;

use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use regex::{Captures, Regex};

pub fn strings_to_xml(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::new(BufWriter::new(File::create(output)?));
    let decl = BytesDecl::new("1.0", Some("UTF-8"), None);
    writer.write_event(Event::Decl(decl))?;
    writer.write_event(Event::Text(BytesText::new("\n")))?;

    writer.write_event(Event::Start(BytesStart::new("resources")))?;
    writer.write_event(Event::Text(BytesText::new("\n")))?;
    let file = File::open(input).expect("Failed to open input file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line_str) = line {
            if let Some((key, value)) = parse_line(&line_str) {
                let mut element = BytesStart::new("string");
                element.push_attribute(("name", &*key));
                writer.write_event(Event::Text(BytesText::new("\t")))?;
                writer.write_event(Event::Start(element))?;
                writer.write_event(Event::Text(BytesText::new(&value)))?;
                writer.write_event(Event::End(BytesEnd::new("string")))?;
                writer.write_event(Event::Text(BytesText::new("\n")))?;
            }
        }
    }
    writer.write_event(Event::End(BytesEnd::new("resources")))?;
    Ok(())
}

fn parse_line(line: &str) -> Option<(String, String)> {
    if !line.starts_with("//") && !line.is_empty() {
        let parts: Vec<&str> = line.splitn(2, " = ").collect();
        if parts.len() == 2 {
            let key = parts[0].trim_end_matches(';').trim_matches('"').to_string();
            let mut value = parts[1].trim_end_matches(';').trim_matches('"').to_string();
            // 将有序的%n$@ 替换为%n$s
            let re = Regex::new(r"%(\d+)\$@").ok()?;
            value = re
                .replace_all(&value, |caps: &Captures| format!("%{}$s", &caps[1]))
                .to_string();
            // 将无序的%@ 替换为%s
            let re_unordered = Regex::new(r"%@").ok()?;
            value = re_unordered
                .replace_all(&value, |_caps: &Captures| "%s")
                .to_string();

            return Some((key, value));
        }
    }
    None
}
