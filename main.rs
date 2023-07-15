extern crate epub;
use::epub::doc::EpubDoc;
use html_parser::Dom;

fn main() {
    // building from ./foo, so need to add /src/
    let mut doc = EpubDoc::new("./src/kafka.epub").unwrap();

    for i in 0..3 {// doc.resources.len() {
        let html = doc.get_current_str().unwrap();
        println!("——————————————————————————————————————————————————");
        println!("{}", html);
        println!("——————————————————————————————————————————————————");
        // println!("{:?}", doc.get_current_str());
        assert!(Dom::parse(&html).is_ok());
        doc.go_next();
    }

    /*doc.go_next();
    doc.go_next();
    let t3 = doc.get_current().unwrap();
    let t5 = str::from_utf8(&t3).unwrap();
    println!("{}", t5);*/

    // println!("{}.xhtml", doc.spine[0]);


    /*for (kS, (vP, vS)) in doc.resources {
        println!("");
    }*/
    /* for (key, (vPath, vString)) in doc.resources {
        println!("{}", key);
        println!("!! {}", vString); // <- how to get the string part
    } */

}
