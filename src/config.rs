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
use anyhow::{Result, Context};

use crate::key_codes::code_from_key_name;
use crate::key_tree::{KTree, KeySequence};

/// Content of key-value store
#[derive(Clone, Debug)]
pub enum KeyValueData {
    Float(f32),
    Int(isize),
    Text(String),
    None,
}

/// Helper that removes double quotes around a str, if there are any
fn strip_quotes_if_any(text: &str) -> &str {
    if let Some(value) = strip_dquotes(text) {
        value
    } else {
        text
    }
}

/// A simple key-value store that holds floats, integers, strings, or
/// empty entries (None)
pub struct KeyValueStore(pub HashMap<String, KeyValueData>);

impl KeyValueStore {
    /// Insert a key/value pair into the store
    ///
    /// The type of data stored is:
    ///
    /// - Text:  if the value is enclosed in double quotes ("value")
    /// - None:  if the value is None (without quotes)
    /// - Float: if the value contains a decimal point ('.')
    /// - Int:   if none of the above applies
    ///
    /// Returns Ok(()) when the value could be parsed
    ///
    pub fn add(&mut self, key: &str, value: &str) -> Result<()> {
        if let Some(value) = strip_dquotes(value) {
            self.0.insert(key.to_string(), KeyValueData::Text(value.to_string()));
        } else if value == "None" {
            self.0.insert(key.to_string(), KeyValueData::None);
        } else if value.starts_with("0x") {
            let val = isize::from_str_radix(value.trim_start_matches("0x"), 16)
                .context(format!("Could not parse value as hex: {}", value))?;
            self.0.insert(key.to_string(), KeyValueData::Int(val));
        } else if value.contains(".") {
            let val = value.parse::<f32>()
                .context(format!("Could not parse value as float: {}", value))?;
            self.0.insert(key.to_string(), KeyValueData::Float(val));
        } else {
            let val = value.parse::<isize>()
                .context(format!("Could not parse this value: {}", value))?;
            self.0.insert(key.to_string(), KeyValueData::Int(val));
        }
        Ok(())
    }

    /// Retrieve a value from the store
    ///
    /// Return the corresponding enum variant, or None if key not found
    ///
    pub fn get(&self, key: &str) -> Option<KeyValueData> {
        self.0.get(key).cloned()
    }

    /// Retrieve a key as Option<f32>
    ///
    /// This works for Float and Int types. For other types or if the
    /// key is not found, None is returned.
    ///
    pub fn get_float(&self, key: &str) -> Option<f32> {
        if let Some(KeyValueData::Float(v)) = self.0.get(key) {
            return Some(*v);
        }
        if let Some(KeyValueData::Int(v)) = self.0.get(key) {
            return Some(*v as f32);
        }
        None
    }

    /// Retrieve a key as Option<&str>
    ///
    /// Returns None if no key is found or its type is different
    ///
    pub fn get_str(&self, key: &str) -> Option<&str> {
        if let Some(KeyValueData::Text(v)) = self.0.get(key) {
            return Some(v);
        }
        None
    }
}

/// Add the definitions found in file `filename` to the key-value
/// store or the tree of keystroke sequences.
///
/// The lines in the file must have one of the following formats:
///
/// - key = value
///   The (key, value) pair is stored in the key-value store
///   The value may be a string (in double quotes), None,
///   a float (with decimal point), or an int
///
/// - code1, code2, code3 => "command"
///   This defines a key sequence that is stored in the KTree
///   The code may be u16 values, or symbolic representations (e.g. KEY_KP0)
///
/// - # comment
///   ; comment
///   // comment
///   Comments and blank lines are ignored
///
/// Returns Ok(()) and Err(_) when the file cannot be
/// opened.  Invalid lines cause an error message to be printed, but
/// the return value remains Ok(())
///
pub fn init_from_file(filename: &str, tree: &mut KTree, kvstore: &mut KeyValueStore) -> Result<()> {
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

    let h = File::open(filename).context(format!("Could not open file {}", filename))?;
    let reader = BufReader::new(h);
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim().is_empty()
                || line.starts_with("#")
                || line.starts_with(";")
                || line.starts_with("//") {
                    // skip comments and empty lines
                } else if let Some(caps) = rx2.captures(&line) {
                    // normal line with key sequence
                    let keys : Vec<String> = caps[1]
                        .split(",")
                        .map(|x| x.trim().to_string())
                        .collect();
                    // replace text tokens by values
                    let keys : Vec<u16> =
                        keys.iter().map(|x| -> u16 {
                            if let Some(v) = code_from_key_name(x) {
                                v
                            } else if let Ok(v) = x.parse::<u16>() {
                                v
                            } else {
                                eprintln!("Cannot interpret key code '{}'", x);
                                0 as u16
                            }
                        }).collect();
                    tree.add(&KeySequence::from(&keys),
                             Some(strip_quotes_if_any(caps[2].trim()).to_string()));
                } else if let Some(caps) = rx1.captures(&line) {
                    // add data to key-value store
                    let key = &caps["key"];
                    let val = &caps["val"];
                    if let Err(msg) = kvstore.add(key.trim(), val.trim()) {
                        eprintln!("Error: {} !", msg);
                    }
                } else {
                    eprintln!("Could not parse line <{:?}>", rx1);
                }
        }
    }
    Ok(())
}
