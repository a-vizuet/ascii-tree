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
    children: Option<Vec<Element>>,
}

fn main() {
    match read_folder(".") {
        Ok(readed_folder) => println!("{0}", init_ascii_build(readed_folder)),
        Err(error) => println!("{:?}", error),
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
                children: None,
            });
        } else {
            elements.push(Element {
                type_el: TypeElement::DIRECTORY,
                name: entry.file_name(),
                children: Some(read_folder(entry.path().to_str().unwrap())?),
            });
        }
    }

    Ok(elements)
}

fn init_ascii_build(elements: Vec<Element>) -> String {
    let mut ascii_tree = String::new();
    ascii_tree.push_str("Main folder\n");
    to_ascii(Some(ascii_tree), elements, 0)
}

fn to_ascii(ascii_tree: Option<String>, elements: Vec<Element>, level: i32) -> String {
    let mut unwrapped_ascii_tree: String = ascii_tree.unwrap_or("".to_string());
    for element in elements {
        unwrapped_ascii_tree.push_str(&"|   ".repeat(level as usize));

        unwrapped_ascii_tree
            .push_str(&format!("|--> {:?} \n", element.name.to_str().unwrap()).to_string());

        if format!("{:?}", element.type_el) == format!("{:?}", TypeElement::DIRECTORY) {
            unwrapped_ascii_tree.push_str(&to_ascii(None, element.children.unwrap(), level + 1));
        }
    }

    unwrapped_ascii_tree.to_string()
}
