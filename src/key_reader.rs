//! Sets up thread that gets keystrokes from keypad

use std::process::Command;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use byteorder::{NativeEndian, ReadBytesExt};

use crate::key_codes::EventType;
use crate::xinput::read_xinput;

pub fn start_key_reader(vid: Option<u16>, pid: Option<u16>)
                        -> Option<(JoinHandle<()>, mpsc::Receiver<u16>)> {
    // make sure the keyboard is present and floating
    let mut xdevs = read_xinput();
    if let Some(vid) = vid {
        xdevs = xdevs
            .into_iter()
            .filter(|x| x.usb_vid == vid)
            .collect();
    }
    if let Some(pid) = pid {
        xdevs = xdevs
            .into_iter()
            .filter(|x| x.usb_pid == pid)
            .collect();
    }
    if xdevs.is_empty() {
        eprintln!("No matching X-input devices found !");
        return None;
    }
    if xdevs.len() > 1 {
        eprintln!("More than 1 X-input device matches criteria !");
        return None;
    }
    let xdev = xdevs.first().unwrap();
    if ! xdev.floating {
        let _ = Command::new("xinput")
            .arg("float")
            .arg(xdev.id.to_string())
            .output()
            .expect("Making device 'float' failed");
    }

    let device_name = xdev.device.clone();

    let (tx, rx): (mpsc::Sender<u16>, mpsc::Receiver<u16>) = mpsc::channel();

    let reader = thread::spawn(move || {
        key_reader_thread(device_name, tx);
    });

    return Some((reader, rx));
}


fn key_reader_thread(dev: String, tx: mpsc::Sender<u16>) {
    eprintln!("Opening {}", dev);
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);
    let mut dev_file = file_options.open(dev).unwrap();

    loop {
        let mut packet = [0u8; 24];
        dev_file.read_exact(&mut packet).unwrap();
        let mut rdr = Cursor::new(packet);
        let tv_sec  = rdr.read_u64::<NativeEndian>().unwrap();
        let tv_usec = rdr.read_u64::<NativeEndian>().unwrap();
        let evtype  = rdr.read_u16::<NativeEndian>().unwrap();
        let code    = rdr.read_u16::<NativeEndian>().unwrap();
        let value   = rdr.read_i32::<NativeEndian>().unwrap();

        if evtype == EventType::EvKey as u16 {
            match code {
                _ => {
                    //println!("code={} value={}", code, value);
                    if value == 1 {
                        tx.send(code).unwrap();
                    }
                }
            }
        } else if evtype == EventType::EvSyn as u16 ||
            evtype == EventType::EvMsc as u16
        {
            // discard event
        } else if evtype == EventType::EvLed as u16 {
            println!("LED={} {}", code, value);
        }
        else {
            println!("{} {} type={} code={} value={} <=", tv_sec, tv_usec, evtype, code, value);
        }
    }

}
