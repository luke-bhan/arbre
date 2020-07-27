extern crate hex;
use hex::FromHex;

use crate::entry::Entry;
use std::fmt;
use std::str;

static MODE: &str = "100644";

pub struct Tree {
    entries: Vec<Entry>
}

impl Tree {
    pub fn new(mut entries: Vec<Entry>) -> Tree {
        entries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Tree{entries}
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res: String = "".to_string();
        for val in self.entries.iter() {
            let packed: Vec<u8> = Vec::from_hex(&val.oid).expect("Invalid Hex String");
            let mut packed_str: String = "".to_string();
            for item in packed {
                packed_str.push(item as char);
            }
            let entry_str = format!("{} {}{}", MODE, val.name, packed_str);
            res = format!("{}{}", res, entry_str);
        }
        write!(f, "{}", res)
    }
}
            






