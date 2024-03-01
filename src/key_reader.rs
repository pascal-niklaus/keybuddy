//! Sets up thread that gets keystrokes from keypad

use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio;
// use tokio::time::{Duration, sleep};
use tokio::sync::mpsc;

use std::io::Cursor;
use byteorder::{NativeEndian, ReadBytesExt};

use crate::key_codes::EventType;
use crate::xinput::read_xinput;

/// Find device based on USB id and make it float, and return device
/// name, or None is an error occurred
pub fn key_device_setup(vid: Option<u16>,
                        pid: Option<u16>)
                        -> Result<String, String>
{
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

    xdevs = xdevs
        .into_iter()
        .filter(|x| !x.name.ends_with("Control"))
        .collect();

    if xdevs.is_empty() {
        return Err("No matching X-input devices found !".to_string());
    }
    if xdevs.len() > 1 {
        return Err("More than 1 X-input device matches criteria !".to_string());
    }
    let xdev = xdevs.first().unwrap();
    if ! xdev.floating {
        let _ = Command::new("xinput")
            .arg("float")
            .arg(xdev.id.to_string())
            .output()
            .expect("Making device 'float' failed");
    }
    Ok(xdev.device.clone())
}


pub async fn key_reader_task(file_name: &String,
                             ev_tx: mpsc::Sender<u16>,
                             mut stop_rx: mpsc::Receiver<()>)
{
    eprintln!("task started: {}", file_name);

    let file = File::open(file_name).await;
    if file.is_err() {
        eprintln!("Could not open file {}", file_name);
        return;
    }
    let mut file = file.unwrap();


    loop {
        let mut packet = [0u8; 24];
        let rd = file.read_exact(&mut packet);

        tokio::select! {
            _ = rd => {
                let mut rdr = Cursor::new(packet);
                let _tv_sec  = ReadBytesExt::read_u64::<NativeEndian>(&mut rdr).unwrap();
                let _tv_usec = ReadBytesExt::read_u64::<NativeEndian>(&mut rdr).unwrap();
                let evtype  = ReadBytesExt::read_u16::<NativeEndian>(&mut rdr).unwrap();
                let code    = ReadBytesExt::read_u16::<NativeEndian>(&mut rdr).unwrap();
                let value   = ReadBytesExt::read_i32::<NativeEndian>(&mut rdr).unwrap();

                if evtype == EventType::EvKey as u16 && value == 1 {
                    ev_tx.send(code).await.unwrap();
                }
            },
            _ = stop_rx.recv() => {
                eprintln!("Stop received");
                break;
            }
        }
    }
}





// fn key_reader_thread(dev: String, tx: mpsc::Sender<u16>) {
//     eprintln!("Opening {}", dev);
//     let mut file_options = OpenOptions::new();
//     file_options.read(true);
//     file_options.write(false);
//     let mut dev_file = file_options.open(dev).unwrap();

//     loop {
//         let mut packet = [0u8; 24];
//         dev_file.read_exact(&mut packet).unwrap();
//         let mut rdr = Cursor::new(packet);
//         let tv_sec  = rdr.read_u64::<NativeEndian>().unwrap();
//         let tv_usec = rdr.read_u64::<NativeEndian>().unwrap();
//         let evtype  = rdr.read_u16::<NativeEndian>().unwrap();
//         let code    = rdr.read_u16::<NativeEndian>().unwrap();
//         let value   = rdr.read_i32::<NativeEndian>().unwrap();

//         if evtype == EventType::EvKey as u16 {
//             match code {
//                 _ => {
//                     if value == 1 {
//                         println!("code={} value={}", code, value);
//                         tx.send(code).unwrap();
//                     }
//                 }
//             }
//         } else if evtype == EventType::EvSyn as u16 ||
//             evtype == EventType::EvMsc as u16
//         {
//             // discard event
//         } else if evtype == EventType::EvLed as u16 {
//             println!("LED={} {}", code, value);
//         }
//         else {
//             println!("{} {} type={} code={} value={} <=", tv_sec, tv_usec, evtype, code, value);
//         }
//     }

// }
