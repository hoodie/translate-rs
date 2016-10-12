extern crate xml;

use std::env;
use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};
use xml::name::OwnedName;

fn search_in_dict(search_term:&str) {
    let path = "../fd-dictionaries/deu-eng/deu-eng.tei";
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let entry_name = "entry";
    let orth_name = "orth";
    let trans_name = "quote";

    let mut in_entry = false;
    let mut in_orth  = false;
    let mut in_trans = false;


    let cmp = |name:&OwnedName, s| name.local_name == s;

    let mut orth = None;
    let mut trans = None;


    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { ref name, .. })
                if cmp(name, entry_name) => in_entry = true,
                Ok(XmlEvent::EndElement { ref name })       
                    if cmp(name, entry_name) => {
                        in_entry = false;
                        if let (&Some(ref orth), &Some(ref trans)) = (&orth,&trans) {
                            println!("{:?} = {:?}\n", orth, trans);
                        }
                        orth = None;
                        trans = None;
                    },

            Ok(XmlEvent::StartElement { ref name, .. }) 
                if in_entry && cmp(name, orth_name) => in_orth = true,
            Ok(XmlEvent::EndElement { ref name })
                if cmp(name, orth_name) => in_orth = false,

            Ok(XmlEvent::StartElement { ref name, .. }) 
                if in_entry && cmp(name, trans_name) => in_trans = true,
            Ok(XmlEvent::EndElement { ref name })
                if cmp(name, trans_name) => in_trans = false,

            Ok(XmlEvent::Characters( ref content )) if in_orth => {
                if content.to_lowercase().contains(search_term) {
                    orth = Some(content.to_owned())
                }
            },
            Ok(XmlEvent::Characters( ref content )) if in_trans => trans = Some(content.to_owned()),

            Err(e) => { println!("Error: {}", e); break; }
            //ref any @ _ if in_orth => println!("{:?}", any),
            _ => {}
        }

    }
}

fn main() {
    if let Some(search_term) = env::args().nth(1) {
        println!("You are looking for: {:?}", search_term);
        search_in_dict(&search_term);
    } else {
        println!("Please enter a search term");
        std::process::exit(1);
    }
}


