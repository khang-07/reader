use std::fs::File;
use serde_json;

pub fn main() {
    let file = File::open("../src-book/foo.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let test = json.as_object().unwrap();
    
    for (title, content) in test {
        println!("{}", title);
        let t = content.as_array().unwrap();
        println!("{:?}", t[0]);
        // use input from main(a, b, c) to access chapter/paragraph/sentence
        // for now focus on design : backend is good enough rn
        // store hashmap somewhere? main.rs maybe
    }

    println!("worked");
}