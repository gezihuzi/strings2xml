use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    // 打开iOS的.strings文件
    let input_path = "./example.strings";
    let input_file = File::open(input_path).expect("无法打开输入文件");

    // 创建输出的strings.xml文件
    let output_path = "./strings.xml";
    let mut output_file = File::create(output_path).expect("无法创建输出文件");

    // 逐行读取.strings文件并写入到strings.xml文件中
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        if let Ok(line) = line {
            // 解析.key = value对
            if let Some(index) = line.find('=') {
                let (key, value) = line.split_at(index);
                let key = key.trim();
                let value = value[1..].trim();

                // 写入<resources>标签和<string>标签到strings.xml文件
                write!(
                    &mut output_file,
                    "<string name=\"{key}\">{value}</string>\n",
                    key = escape_xml_entities(key),
                    value = escape_xml_entities(value)
                )
                .expect("无法写入到输出文件");
            }
        }
    }

    println!("转换完成！");
}

fn escape_xml_entities(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
