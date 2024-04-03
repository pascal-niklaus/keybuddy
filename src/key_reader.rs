//! Sets up thread that gets keystrokes from keypad

use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

use std::io::Cursor;
use byteorder::{NativeEndian, ReadBytesExt};
use anyhow::{Result, Context, bail};

use crate::key_codes::EventType;
use crate::xinput::read_xinput;

/// Find device based on USB id and a predicate based on the device's
/// name (the one listed by e.g. lsusb), Make the device float, and
/// return device name, or None is an error occurred
///
pub fn key_device_setup(vid: Option<u16>,
                        pid: Option<u16>,
                        dev_filter: Option<Box<dyn Fn(&str)->bool>>)
                        -> Result<String>
{
    // get device list and filter it
    let mut xdevs = read_xinput()
        .context("getting devices listed by xinput")?;
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

    if let Some(fun) = dev_filter {
    xdevs = xdevs
        .into_iter()
        .filter(|x| fun(&x.name))
        .collect();
    }

    if xdevs.is_empty() {
        bail!("No matching X-input devices found !");
    }
    if xdevs.len() > 1 {
        bail!(format!("Only 1 x-input device should match, but found {}", xdevs.len()));
    }
    let xdev = xdevs.first().unwrap();
    if ! xdev.floating {
        let _ = Command::new("xinput")
            .arg("float")
            .arg(xdev.id.to_string())
            .output()
            .context("Making device 'float'")?;
    }
    Ok(xdev.device.clone())
}

/// Task waits for key strokes and feeds these into ev_tx. It aborts
/// when anything is received via stop_rx.
///
pub async fn key_reader_task(file_name: &String,
                             ev_tx: mpsc::Sender<u16>,
                             mut stop_rx: mpsc::Receiver<()>)
{
    let file = File::open(file_name).await;
    if file.is_err() {
        eprintln!("Could not open device {}", file_name);
        return;
    }
    let mut file = file.unwrap();
    eprintln!("Listening on device {} ...", file_name);

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
                break;
            }
        }
    }
}
