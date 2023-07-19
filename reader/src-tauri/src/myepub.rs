use epub::doc::EpubDoc;
use html_parser::{Dom, Result};
use serde_json::{self, json};
use html2text;
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::{fs, io::BufReader, fmt::format};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    title: String,
    content: Vec<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    title: String,
    content: Vec<String>
}

pub fn get_chapter(html: &String) -> Result<Chapter> {
    let content_html = html;
    let content_bytes: &[u8] = content_html.as_bytes();
    let content: String = html2text::from_read(&content_bytes[..], 10000);
    let content_paragraphs = content.split("\n\n\n").collect::<Vec<&str>>();
    let mut content_sentences = Vec::new();

    for paragraph in content_paragraphs {
        // The last sentence in a paragraph (vector of sentences) will have a period.
        // The last word in a chapter is a "\n"
        let sentences = paragraph.split(". ").collect::<Vec<&str>>();
        let mut sentences_string = Vec::new();

        for sentence in sentences {
            sentences_string.push(sentence.to_string());
        }
        content_sentences.push(sentences_string);
    }

    let title = &content_sentences[0][0];
    let re = Regex::new(r"[^A-Za-z0-9 ]").unwrap();
    let title_clean = &re.replace_all(&title, " ").to_string();
    let title_trim = title_clean.trim().to_string();

    let chapter = Chapter {
        title: title_trim,
        content: content_sentences
    };

    Ok(chapter)    
}

pub fn book_to_json(path: &str) {
    let mut doc = EpubDoc::new(path).unwrap();
    let chapter_count = doc.get_num_pages();

    // skip content.opf and title page
    let _ = doc.go_next();
    let _ = doc.go_next();

    let mut prejsons = Vec::new();

    for i in 0..2 { // chapter_count {
        let html = doc.get_current_str().unwrap();
        let chapter = get_chapter(&html).unwrap();
        let prejson = format!("\"{}\" : [{}]", chapter.title, serde_json::to_string_pretty(&chapter.content).unwrap());
        prejsons.push(prejson);
        let _ = doc.go_next();
    }

    let test = format!("{{ \n{} \n}}", prejsons.join(",\n"));

    // let rel_path = PathBuf::from("../src/");
    // let abs_path =  fs::canonicalize(&srcdir);

    let _ = fs::write("../src-book/foo.json", test);
    println!("epub loaded");
}

pub fn main() {
    let path = "/Users/khangnguyen/Code/testing epub rust/foo/src/kafka.epub";
    book_to_json(path);
}
