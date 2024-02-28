use std::fs::OpenOptions;
use std::io::{Cursor, Read};
use byteorder::{NativeEndian, ReadBytesExt};
use input::ffi::libinput_event_pointer_get_axis_value;

mod key_codes;
mod xinput;

use crate::key_codes::{RawKeyCode, RawCodes};
use crate::xinput::XinputEntry;

#[repr(u16)]
#[derive(Clone, Copy)]
enum EventType {
    EvSyn      = 0x00,
    EvKey      = 0x01,
    EvRel      = 0x02,
    EvAbs      = 0x03,
    EvMsc      = 0x04,
    EvSw       = 0x05,
    EvLed      = 0x11,
    EvSnd      = 0x12,
    EvRep      = 0x14,
    EvFf       = 0x15,
    EvPwr      = 0x16,
    EvFfStatus = 0x17,
    EvMax      = 0x1f,
    EvCnt      = 0x20,
}

/*
 * value: 0 = release, 1=press, 2=still_pressed (repeat)
 *
 */


fn main() {
    // get all input devices
    let x = xinput::read_xinput();

    // extract name of keyboard to read from
    let k : Vec<XinputEntry> = x
        .into_iter()
        .filter(|x| x.usb_vid == 1241)
        .filter(|x| ! x.name.contains("Control"))
        .collect();
    eprintln!("{:?}", k);

    // if it is not floating, make it floating
    //  xinput float id


    // open the device file
    let mut file_options = OpenOptions::new();
    file_options.read(true);
    file_options.write(false);
    let mut dev_file = file_options.open("/dev/keypad_event16").unwrap();


    let mut shift_left = false;
    let mut shift_right = false;
    let mut ctrl_left = false;
    let mut ctrl_right = false;
    let mut alt_left = false;
    let mut alt_right = false;
    let mut meta_left = false;
    let mut meta_right = false;
    let mut shift = false;
    let mut alt = false;
    let mut ctrl = false;
    let mut meta = false;

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
                RawCodes::KeyLeftShift  => {
                    shift_left = value != 0;
                    shift = shift_left || shift_right
                },
                RawCodes::KeyRightShift => {
                    shift_right = value != 0;
                    shift = shift_left || shift_right
                },
                RawCodes::KeyLeftCtrl  => {
                    ctrl_left = value != 0;
                    ctrl = ctrl_left || ctrl_right;
                },
                RawCodes::KeyRightCtrl => {
                    ctrl_right = value != 0;
                    ctrl = ctrl_left || ctrl_right;
                },
                RawCodes::KeyLeftAlt  => {
                    alt_left = value != 0;
                    alt = alt_left || alt_right;
                },
                RawCodes::KeyRightAlt => {
                    alt_right = value != 0;
                    alt = alt_left || alt_right;
                },
                RawCodes::KeyLeftMeta  => {
                    meta_left = value != 0;
                    meta = meta_left || meta_right;
                },
                RawCodes::KeyRightMeta => {
                    meta_right = value != 0;
                    meta = meta_left || meta_right;
                },
                _ => {
                    println!("{} {} type={} code={} value={}", tv_sec, tv_usec, evtype, code, value);
                }
            }
        } else if evtype == EventType::EvSyn as u16 ||
            evtype == EventType::EvMsc as u16
        {
            // discard
        } else if evtype == EventType::EvLed as u16 {
            println!("LED={} {}", code, value);
        }
        else {
            println!("{} {} type={} code={} value={} <=", tv_sec, tv_usec, evtype, code, value);
        }
    }
}
