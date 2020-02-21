use std::ffi::OsString;
use std::fs::{read_dir, DirEntry};
use std::io::Error;

#[derive(Debug)]
enum TypeElement {
    FILE,
    DIRECTORY,
}

#[derive(Debug)]
struct Element {
    type_el: TypeElement,
    name: OsString,
    childs: Option<Vec<Element>>,
}

fn main() {
    match read_folder(".") {
        Ok(a) => println!("{:?}", a),
        Err(e) => println!("{:?}", e),
    }
}

fn read_folder(path: &str) -> Result<Vec<Element>, Error> {
    let mut elements: Vec<Element> = vec![];
    for entry in read_dir(path)? {
        let entry: DirEntry = entry?;
        if entry.metadata().unwrap().is_file() {
            elements.push(Element {
                type_el: TypeElement::FILE,
                name: entry.file_name(),
                childs: None,
            });
        } else {
            elements.push(Element {
                type_el: TypeElement::DIRECTORY,
                name: entry.file_name(),
                childs: Option::from(read_folder(entry.path().to_str().unwrap())?),
            });
        }
    }

    Ok(elements)
}
