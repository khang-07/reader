use epub::doc::EpubDoc;
use html_parser::{Dom, Result};
use serde_json;

pub fn format(value: String) -> String {
    let test = value.replace("\\n", "").replace("\\", "");
    let mut chars = test.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn get_content(html: &String) -> Result<Vec<String>> {
    let json = Dom::parse(&html)?.to_json_pretty()?;
    let result: serde_json::Value = serde_json::from_str(&json)?;
    let data = &result["children"][0]["children"][1]["children"].as_array().unwrap();
    let mut contents = Vec::new();

    for i in 0..data.len() {
        if data[i]["classes"][0] == "calibre8" {
            let line = data[i + 1].to_string();
            let formatted = format(line.clone());
            contents.push(formatted);
        }
    }

    contents.pop();

    Ok(contents)
}

pub fn get_book(path: &str) -> Vec<Vec<String>> {
    let mut doc = EpubDoc::new(path).unwrap();
    let mut chapters = Vec::new();
    let chapter_count = doc.get_num_pages();

    for i in 1..chapter_count {
        let html = doc.get_current_str().unwrap();
        chapters.push(get_content(&html).unwrap());
        let _ = doc.go_next();
        println!("{i}");
    }

    chapters
}

pub fn main() {
    // building from ./foo, so need to add /src/
    let chapters = get_book("/Users/khangnguyen/Code/testing epub rust/foo/src/kafka.epub");
    println!("{:?}", chapters[2]);
}
