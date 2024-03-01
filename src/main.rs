use std::collections::HashMap;
use std::process::ExitCode;
use config::{KeyValueData, KeyValueStore};
use key_tree::{KTree, KeySequence};
use tokio::sync::mpsc;
use tokio::time::Instant;

use std::process::Command;
use argparse::{ArgumentParser, StoreTrue, Store};

mod key_tree;
mod key_codes;
mod xinput;
mod key_reader;
mod config;

use crate::key_reader::{key_device_setup, key_reader_task};
use crate::key_codes::key_name_from_code;
use crate::config::init_from_file;

fn exec_gromit(opt: &str) {
    let _ = Command::new("gromit-mpx")
        .arg(opt)
        .output()
        .expect("gromit-mpx should be found!");
}

/// clear key queue after timeout
async fn key_state_machine(mut rx: mpsc::Receiver<u16>) {

    let mut seq : Vec<u16> = vec![];

    let mut now = Instant::now();

    loop {
        if let Some(k) = rx.recv().await {
            if let Some(name) = key_name_from_code(k) {
                eprintln!("{} ",name);
            } else {
                eprintln!("k={}", k);
            }
            if now.elapsed().as_secs() > 2 {
                seq.clear();
            }
            now = Instant::now();
            seq.push(k);

            // if cmp(&seq, &vec![1, 1, 1]) {
            //     break;
            // }
            // if cmp(&seq, &vec![29]) {
            //     exec_gromit("-t");
            //     seq.clear();
            // }
            // if cmp(&seq, &vec![56]) {
            //     exec_gromit("-c");
            //     seq.clear();
            // }
            // if cmp(&seq, &vec![69]) {
            //     exec_gromit("-v");
            //     seq.clear();
            // }
            eprintln!("seq={:?}", seq);

        } else {
            break;
        }
    }
}

#[derive(Debug)]
struct Options {
    cfg_file : String,
    show_keys : bool,
}

impl Options {
    pub fn new() -> Self {
        Self {
            //cfg_file: "~/.config/keymacros/keymacros.cfg".to_string(),
            cfg_file: "/data/_programming/keymacros/keymacros/keymacros.conf".to_string(),
            show_keys: false,
        }
    }
}

fn parse_args() -> Options {
    let mut opts = Options::new();
    let k = format!("Name of config file (default: {})", opts.cfg_file);
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("keymacros V0.1 -- (C) 2024 Pascal Niklaus");
        ap.refer(&mut opts.show_keys)
            .add_option(&["-k", "--show-key"], StoreTrue, "Show key strokes received");
        ap.refer(&mut opts.cfg_file)
            .add_option(&["--config"], Store, k.as_str());
        ap.parse_args_or_exit();
    }
    opts
}


#[tokio::main]
async fn main() -> ExitCode {

    // command-line arguments
    let opts = parse_args();
    eprintln!("{:?}", &opts);

    // read config file
    let mut kt = KTree::new();
    let mut kv = KeyValueStore(HashMap::<String, KeyValueData>::new());
    if let Err(msg) = init_from_file(&opts.cfg_file, &mut kt, &mut kv) {
        eprintln!("Error: {}", msg);
    }

    // collect all input devices
    let mut vid = None;
    if let Some(KeyValueData::Int(v)) = kv.get("vid") {
        vid = Some(v as u16);
    }
    let mut pid = None;
    if let Some(KeyValueData::Int(v)) = kv.get("pid") {
        pid = Some(v as u16);
    }

    // filter devices, make the one found float, and extracts its name
    let dev_name = key_device_setup(vid, pid);
    if let Err(err) = dev_name {
        eprintln!("An error occurred: {}", err);
        return ExitCode::FAILURE;
    }
    let dev_name = dev_name.unwrap().to_string();

    // communication channels
    let (ev_tx, mut ev_rx) = mpsc::channel::<u16>(10);
    let (stop_tx, stop_rx) = mpsc::channel::<()>(1);

    // spawn keystroke reader
    let task_handle = tokio::spawn(async move {
        key_reader_task(&dev_name, ev_tx, stop_rx).await;
    });


    // show key strokes
    if opts.show_keys {
        eprintln!("Showing codes of key strokes received (Ctrl-C to abort)");
        while let Some(k) = ev_rx.recv().await {
            if let Some(name) = key_name_from_code(k) {
                eprint!("{} ",name);
            } else {
                eprint!("{}", k);
            }
        }
        return ExitCode::SUCCESS;
    }


    {
        let mut seq : Vec<u16> = vec![];
        let mut now = Instant::now();

        loop {
            if let Some(k) = ev_rx.recv().await {
                if let Some(name) = key_name_from_code(k) {
                    eprintln!("{} ",name);
                } else {
                    eprintln!("k={}", k);
                }
                if now.elapsed().as_secs() > 2 {
                    seq.clear();
                }
                now = Instant::now();
                seq.push(k);

                // if cmp(&seq, &vec![1, 1, 1]) {
                //     break;
                // }
                // if cmp(&seq, &vec![29]) {
                //     exec_gromit("-t");
                //     seq.clear();
                // }
                // if cmp(&seq, &vec![56]) {
                //     exec_gromit("-c");
                //     seq.clear();
                // }
                // if cmp(&seq, &vec![69]) {
                //     exec_gromit("-v");
                //     seq.clear();
                // }
                eprintln!("seq={:?}", seq);

            } else {
                break;
            }
        }
    }
    /// spawn key
    // let key_handle = tokio::spawn(async move {
    //     key_state_machine(ev_rx).await;
    // });

    // key_handle.await.unwrap();

    stop_tx.send(()).await.unwrap();
    task_handle.await.unwrap();

    ExitCode::SUCCESS
}
