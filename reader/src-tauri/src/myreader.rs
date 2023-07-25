use std::fs::File;
use serde_json;
use serde::{Serialize, Deserialize};
use alphanumeric_sort;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chapter {
    title: String,
    content: Vec<Vec<String>>
}

pub fn get_chapter(chapter_title: &str) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let file = File::open("../src-book/foo.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let test = json.as_object().unwrap();
    for (title, content) in test {
        if title == chapter_title {
            println!("pee{}", title);
            // println!("{:?}", serde_json::from_value::<Vec<Vec<String>>>(content.clone()).unwrap());
            let result = serde_json::from_value::<Vec<Vec<String>>>(content.clone()).unwrap();
            println!("{:?}", result);
        }
    }
    result
}

pub fn get_chapter_titles() -> Vec<String> {
    let mut result = Vec::new();
    let file = File::open("../src-book/foo.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let test = json.as_object().unwrap();
    for (title, content) in test {
        result.push(title.to_string());
    }
    result.sort();
    result
}

pub fn main() {
    println!("worked");
}