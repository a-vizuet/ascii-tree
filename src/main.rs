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
        Ok(a) => println!("{0}", to_ascii(a)),
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

/**
 * It should return something like
 * Main folder
 * |--> Secondary folder
 * |    |-> File
 */
fn to_ascii(elements: Vec<Element>) -> String {
    let mut ascii_tree = String::new() + "Main folder\n";

    for element in elements {
        ascii_tree += "|";
        match element.type_el {
            TypeElement::FILE => {
                ascii_tree += &format!("--> {:?} \n", element.name.to_str().unwrap()).to_string();
            }
            TypeElement::DIRECTORY => {
                ascii_tree += &format!("--> {:?} \n", element.name.to_str().unwrap()).to_string()
            }
        }
    }

    ascii_tree.to_string()
}
