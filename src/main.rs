use std::collections::HashMap;
use std::process::ExitCode;
use std::process::Command;

use clap::Parser;
use anyhow::{Result, Context};
use tokio::sync::mpsc;
use tokio::time::Instant;

mod key_tree;
mod key_codes;
mod xinput;
mod key_reader;
mod config;

use config::{KeyValueData, KeyValueStore};

use key_reader::{key_device_setup, key_reader_task};
use key_codes::key_name_from_code;
use key_tree::{KTree, KeySequence};
use config::init_from_file;

/// Execute a command and return true if ok, otherwise false
///
/// The command is passed as single str including all the arguments.
///
fn exec_command(cmd: &str) -> bool {
    if let Some(parts) = shlex::split(cmd) {
        if let Some((cmd, args)) = parts.split_first() {
            return Command::new(cmd).args(args).output().is_ok()
        }
    }
    false
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(about = "KeyBuddy -- keystroke interpreter for separate keypad\n(C) 2024 Pascal Niklaus", long_about = None)]
struct Args {
    /// Show key strokes received
    #[arg(short='k', long="show-keys", default_value_t  = false)]
    show_keys: bool,

    /// Set maximum time span between keystrokes that form a sequence
    #[arg(short='d', long="delay", id="SECONDS", default_value_t = 2.0)]
    key_memory_span: f32,

    /// Use config file
    #[arg(long, default_value_t = std::env::var("HOME").unwrap()+"/.config/keybuddy.conf")]
    cfg_file: String,

    /// Be verbose (for debugging)
    #[arg(short='v', long="verbose", default_value_t = false)]
    debug: bool,
}


#[tokio::main]
async fn main() -> Result<ExitCode> {
    // command-line arguments
    let mut opts = Args::parse();

    eprintln!("KeyBuddy -- (C) 2024 Pascal Niklaus");

    // read config file
    let mut kt = KTree::new();
    let mut kv = KeyValueStore(HashMap::<String, KeyValueData>::new());
    init_from_file(&opts.cfg_file, &mut kt, &mut kv).context("Reading config file")?;

    if let Some(delay) = kv.get_float("delay") {
        opts.key_memory_span = delay;
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
    let mut f1 : Box<dyn Fn(&str)->bool> = Box::new(|_x: &str| -> bool { true });
    let mut f2 : Box<dyn Fn(&str)->bool> = Box::new(|_x: &str| -> bool { true });
    if let Some(KeyValueData::Text(v)) = kv.get("device_include") {
        f1 = Box::new(move |x: &str| -> bool { x.contains(v.as_str()) });
    }
    if let Some(KeyValueData::Text(v)) = kv.get("device_exclude") {
        f2 = Box::new(move |x: &str| -> bool { !x.contains(v.as_str()) });
    }

    // filter devices, make the one found float, and extracts its name
    let dev_name = key_device_setup(vid, pid, Some(Box::new(move |x| { f1(x) & f2(x) })));
    if let Err(err) = dev_name {
        eprintln!("An error occurred: {}", err);
        return Ok(ExitCode::FAILURE);
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
        return Ok(ExitCode::SUCCESS);
    }

    {
        let mut seq : Vec<u16> = vec![];
        let mut now = Instant::now();
        let quit = kv.get_str("quit_command");

        if opts.debug {
            kt.dump();
        }

        let mut newline = true;
        loop {
            if let Some(k) = ev_rx.recv().await {
                if now.elapsed().as_secs_f32() > opts.key_memory_span && !newline {
                    seq.clear();
                    if opts.debug {
                        eprintln!("... aborted");
                    }
                }
                if opts.debug {
                    if let Some(name) = key_name_from_code(k) {
                        eprint!("{} ",name);
                    } else {
                        eprint!("k={}", k);
                    }
                }
                now = Instant::now();
                seq.push(k);

                if let Some(cmd) = kt.find(&KeySequence::from(&seq)) {
                    if quit.is_some() && cmd == quit.unwrap() {
                        if opts.debug {
                            eprintln!("-> exiting...");
                        }
                        break;
                    }
                    if opts.debug {
                        eprintln!("-> executing <{}>", cmd);
                    }
                    exec_command(cmd);
                    seq.clear();
                    newline = true;
                } else {
                    newline = false;
                }
            } else {
                break;
            }
        }
    }

    stop_tx.send(()).await.unwrap();
    task_handle.await.unwrap();

    Ok(ExitCode::SUCCESS)
}
