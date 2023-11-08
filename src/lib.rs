use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use regex::{Captures, Regex};

pub fn strings_to_xml(input: &Path, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let xml = convert_to_xml(&input)?;
    let mut output = File::create(output)?;
    write!(output, "{}", xml)?;
    Ok(())
}

fn convert_to_xml(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(path).expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut xml = "".to_string();
    // 遍历文件的每一行
    for line in reader.lines() {
        if let Ok(line_str) = line {
            // 解析每一行，提取出key和value
            if let Some((key, value)) = parse_line(&line_str) {
                // 将key和value添加到JSON对象中
                xml += &format!(
                    "<string name=\"{}\">{}</string>\n",
                    escape_xml_entities(&key),
                    escape_xml_entities(&value)
                );
            }
        }
    }
    Ok(xml)
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

fn escape_xml_entities(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
