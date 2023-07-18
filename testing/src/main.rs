use std::{collections::HashMap, io, rc::Rc};

use maplit::hashmap;

fn main() {
    let mut input = String::new();
    let language: String;
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read file");

    match input.trim() {
        "de" => {
            language = translation_map(Languages::GERMAN);
        }
        "en" => {
            language = translation_map(Languages::ENGLISH);
        }
        _ => {
            language = translation_map(Languages::GERMAN);
        }
    }

    println!("{}", language.as_str());
}

fn translation_map(language: Languages) -> String {
    let en_us: Rc<HashMap<&'static str, &'static str>> = Rc::new(hashmap! {
        "TEST" => "test TeST"
    });
    let de_de: Rc<HashMap<&'static str, &'static str>> = Rc::new(hashmap! {
        "TEST" => "deutscher test"
    });

    #[allow(unused_assignments)]
    // unused because showcase prototype
    let mut current_translation = Rc::clone(&en_us);
    match language {
        Languages::ENGLISH => {
            current_translation = Rc::clone(&en_us);
        }
        Languages::GERMAN => current_translation = Rc::clone(&de_de),
    }

    current_translation["TEST"].to_owned()
}

enum Languages {
    GERMAN,
    ENGLISH,
}
