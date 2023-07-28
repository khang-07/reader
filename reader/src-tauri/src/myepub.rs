use epub::doc::EpubDoc;
use html_parser::{Result};
use serde_json::{self};
use html2text;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    title: String,
    content: Vec<Vec<String>>
}

pub fn get_chapter(html: &String, index: usize) -> Result<Chapter> {
    let content_html = html;
    let content_bytes: &[u8] = content_html.as_bytes();
    let content_string: String = html2text::from_read(&content_bytes[..], 10000);
    let content_paragraphs = content_string.split(". ").collect::<Vec<&str>>();
    let mut content_sentences = Vec::new();

    for paragraph in content_paragraphs {
        // The last sentence in a paragraph (vector of sentences) will have a period.
        // The last word in a chapter is a "\n"
        let sentences = paragraph.split("\"").collect::<Vec<&str>>();
        let mut sentences_string = Vec::new();

        for sentence in sentences {
            sentences_string.push(sentence.to_string());
        }
        content_sentences.push(sentences_string);
    }

    // let title = &content_sentences[0][0];
    // let re = Regex::new(r"[^A-Za-z0-9 ]").unwrap();
    // let title_clean = &re.replace_all(&title, " ").to_string();
    // let title_trim = title_clean.trim().to_string();

    let title_trim = format!("Chapter {}", index);
    println!("{}", title_trim);

    let chapter = Chapter {
        title: title_trim,
        content: content_sentences
    };

    Ok(chapter)    
}

pub fn book_to_json(path: &str) {
    let mut doc = EpubDoc::new(path).unwrap();
    let chapter_count = doc.get_num_pages();
    let mut chapter_jsons = Vec::new();

    // skip content.opf and title page
    let _ = doc.go_next();
    let _ = doc.go_next();

    for i in 0..chapter_count {
        let html = doc.get_current_str().unwrap();
        let chapter = get_chapter(&html, i).unwrap();
        // println!("From myepub.rs: {}", chapter.title);
        let chapter_json = format!("\"{}\" : {}", chapter.title, serde_json::to_string_pretty(&chapter.content).unwrap());
        chapter_jsons.push(chapter_json);
        let _ = doc.go_next();
    }

    let final_json = format!("{{ \n{} \n}}", chapter_jsons.join(",\n"));
    let _ = fs::write("../src-book/foo.json", final_json);

    // println!("epub loaded");
}

pub fn main() {
    let path = "/Users/khangnguyen/Documents/Books/Inferno.epub";
    book_to_json(path);
}
