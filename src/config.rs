//! Application configuration
//!
//! Data are stored in a key-value store
//! Key sequence data and associted commands in KTree

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::sync::OnceLock;
use std::collections::HashMap;
use regex::Regex;
use quoted_string::strip_dquotes;

use crate::key_tree::{KTree, KeySequence};

#[derive(Clone, Debug)]
pub enum KeyValueData {
    Float(f32),
    Int(isize),
    Text(String),
    None,
}

/// A simple key-value store that holds floats, integers, strings, or
/// Empty entries (None)
///
pub struct KeyValueStore(pub HashMap<String, KeyValueData>);

impl KeyValueStore {
    /// Insert a key/value pair into the store
    ///
    /// If the value is enclosed in double quotes, it is enum variant
    /// Text; if it contains a decimal ('.'), it is parsed as enum
    /// variant Float; if it equals "None", it is stored as enum
    /// variant None; otherwise, it is parsed as integer (enum variant
    /// Int)
    ///
    pub fn add(&mut self, key: &str, value: &str) -> Result<(), String> {
        if let Some(value) = strip_dquotes(value) {
            self.0.insert(key.to_string(), KeyValueData::Text(value.to_string()));
        } else if value == "None" {
            self.0.insert(key.to_string(), KeyValueData::None);
        } else if value.starts_with("0x") {
            if let Ok(v) = isize::from_str_radix(value.trim_start_matches("0x"), 16) {
                self.0.insert(key.to_string(), KeyValueData::Int(v));
            } else {
                return Err(format!("Could not parse value as hex: {}", value));
            }
        } else if value.contains(".") {
            if let Ok(val) = value.parse::<f32>() {
                self.0.insert(key.to_string(), KeyValueData::Float(val));
            } else {
                return Err(format!("Could not parse value as float: {}", value));
            }
        } else if let Ok(val) = value.parse::<isize>() {
            self.0.insert(key.to_string(), KeyValueData::Int(val));
        } else {
            return Err(format!("Could not parse this value: {}", value));
        }
        Ok(())
    }

    /// retrieve a value from the store and return the corresponding
    /// enum variant, or None if the keay was not found.
    ///
    pub fn get(&self, key: &str) -> Option<KeyValueData> {
        self.0.get(key).cloned()
    }
}

/// Add the definitions found in file `filename` to the key-value
/// store or the tree of keystroke sequences.
///
/// the lines have the format:
/// key = value
/// code1, code2, code3 => "command"
/// # comment
/// ; comment
/// // comment
///
pub fn init_from_file(filename: &str, tree: &mut KTree, kvstore: &mut KeyValueStore) -> Result<(), String> {
    // match assignments
    static RX1: OnceLock<Regex> = OnceLock::new();
    let rx1 = RX1.get_or_init(
        || Regex::new(r##"^(?P<key>[a-zA-Z][a-zA-Z_0-9]*)\s*=\s*(?P<val>.+)$"##).unwrap()
    );

    // split line at "->"
    static RX2: OnceLock<Regex> = OnceLock::new();
    let rx2 = RX2.get_or_init(
        || Regex::new(r##"^(.+?)\s*[=-]{1,2}>\s*(Quit|".+")$"##).unwrap()
    );

    if let Ok(h) = File::open(filename) {
        let reader = BufReader::new(h);
        for line in reader.lines() {
            if let Ok(line) = line {
                eprintln!("line: <{}>",line);
                if line.trim().is_empty()
                    || line.starts_with("#")
                    || line.starts_with(";")
                    || line.starts_with("//") {
                    // skip comments and empty lines
                } else if let Some(caps) = rx1.captures(&line) {
                    // add data to key-value store
                    let key = &caps["key"];
                    let val = &caps["val"];
                    if let Err(msg) = kvstore.add(key.trim(), val.trim()) {
                        eprintln!("Error: {} !", msg);
                    }
                } else if let Some(caps) = rx2.captures(&line) {
                    // normal line with key sequence
                    eprintln!("caps: {:?}   ->   {:?}", &caps[1], &caps[2]);
                    let keys : Vec<String> = caps[1].split(",").map(|x| x.trim().to_string()).collect();
                    // replace text tokens by values
                    let keys : Vec<u16> = keys.iter().map(|x| x.parse::<u16>().unwrap()).collect();
                    tree.add(&KeySequence::from(&keys), Some(caps[2].trim().to_string()));
                } else {
                    eprintln!("Could not parse line <{:?}>", rx1);
                }
            }
        }
        Ok(())
    } else {
        Err("Could not open file.".to_string())
    }
}
