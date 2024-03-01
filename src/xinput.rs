use std::process::Command;
use std::str;
use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct XinputEntry {
    pub name: String,
    pub id: usize,
    pub keyboard: bool,
    pub pointer: bool,
    pub slave: bool,
    pub master: bool,
    pub floating: bool,
    pub device: String,
    pub usb_vid: u16,
    pub usb_pid: u16,
}

impl XinputEntry {
    pub fn new_from_line(line: &str) -> Option<Self> {
        static RX: OnceLock<Regex> = OnceLock::new();
        let rx = RX.get_or_init(
            || Regex::new(r"^[^A-Za-z]+(?<name>.+?)\s+id=(?<id>[0-9]+)\s+\[(?<slave>.+)\]").unwrap());
        if let Some(caps) = rx.captures(line) {
            let slave = &caps["slave"];
            Some(Self {
                name: caps["name"].to_string(),
                id: caps["id"].parse::<usize>().unwrap_or(0),
                keyboard: slave.contains("keyboard"),
                pointer: slave.contains("pointer"),
                slave: slave.contains("slave"),
                master: slave.contains("master"),
                floating: slave.contains("floating"),
                device: "".to_string(),
                usb_pid: 0,
                usb_vid: 0,
            })
        } else {
            None
        }
    }
}

pub fn read_xinput() -> Vec<XinputEntry> {
    let mut result = vec![];

    static RX1: OnceLock<Regex> = OnceLock::new();
    let rx1 = RX1.get_or_init(
        || Regex::new(r##"^\s*Device Node.+:\s*"(?<name>.+?)""##).unwrap()
    );

    static RX2: OnceLock<Regex> = OnceLock::new();
    let rx2 = RX2.get_or_init(
        || Regex::new(r##"^\s*Device Product ID.+:\s*(?<vid>[0-9]+)\s*,\s*(?<pid>[0-9]+)"##).unwrap()
    );

    // collect lines output by xinput --list
    let output = Command::new("xinput")
        .arg("-list")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        if let Ok(out) = str::from_utf8(&output.stdout) {
            for line in out.lines() {
                if let Some(ref mut x) = XinputEntry::new_from_line(line) {
                    // add info from xinput --list-props to entry
                    let output = Command::new("xinput")
                        .arg("--list-props")
                        .arg(x.id.to_string())
                         .output()
                        .expect("get props failed");
                    if output.status.success() {
                        if let Ok(out) = str::from_utf8(&output.stdout) {
                            for line in out.lines() {
                                if let Some(caps) = rx1.captures(line) {
                                    x.device = caps["name"].to_string();
                                } else if let Some(caps) = rx2.captures(line) {
                                    x.usb_vid = caps["vid"].parse::<u16>().unwrap_or(0);
                                    x.usb_pid = caps["pid"].parse::<u16>().unwrap_or(0);
                                }
                            }
                        }
                    }
                    result.push(x.clone());
                } else {
                    eprintln!("Cannot parse line <{}>", line);
                }
            }
        }
    } else {
        eprintln!("Failure running 'xlist'");
    }
    return result;
}
