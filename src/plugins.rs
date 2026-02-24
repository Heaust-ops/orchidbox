use std::{fs::read_dir, path::Path, process::Command, thread};

use crate::{config::Config, simulation::send_combo};

pub fn run_every_service(plugins_dir: String, cfg: &Config) {
    let read = read_dir(plugins_dir);
    if let Err(e) = read {
        eprintln!("error opening the plugins directory: {}", e);
        return;
    }

    let read = read.expect("Unhandled error reading the plugins directory.");

    for entry in read {
        if let Err(e) = entry {
            eprintln!("error reading entry: {}", e);
            continue;
        }

        let entry = entry.expect("Unhandled error reading entries from the plugin directory");

        let path = entry.path();

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(".service") && path.is_file() {
                println!("Starting {}", name);
                let args = cfg.get_plugin_args(name.to_string());

                thread::spawn(move || {
                    let status = Command::new(&path).args(args).status();
                    if let Err(e) = status {
                        eprintln!("Error starting the service: {}", e);
                    }
                });
            }
        }
    }
}

pub fn run_plugin_and_send_combo(input: &String, cfg: &Config) {
    // Split on whitespace. If you need shell-style quoting support, see note below.
    let mut parts = input.split("+").map(|e| e.to_string());

    let bin_token = parts.next().expect("failed to parse plugin command");
    let binname = &bin_token[1..]; // strip leading '@'

    let config_args = cfg.get_plugin_args(binname.to_string());
    let mut args: Vec<String> = parts.collect();
    args.extend_from_slice(&config_args);

    // Build executable path: ./plugins/<binname>
    let exe_path = Path::new("./plugins").join(binname);

    if !exe_path.exists() {
        return;
    }
    let output;
    if binname.ends_with(".js") {
        args.insert(0, exe_path.to_string_lossy().to_string());

        // Run and capture stdout (waits for exit)
        output = match Command::new("node").args(&args).output() {
            Ok(o) => o,
            Err(err) => {
                eprintln!("Failed to run plugin {}: {}", exe_path.display(), err);
                return;
            }
        };
    } else if binname.ends_with(".ts") {
        args.insert(0, exe_path.to_string_lossy().to_string());

        // Run and capture stdout (waits for exit)
        output = match Command::new("ts-node").args(&args).output() {
            Ok(o) => o,
            Err(err) => {
                eprintln!("Failed to run plugin {}: {}", exe_path.display(), err);
                return;
            }
        };
    } else {
        // Run and capture stdout (waits for exit)
        output = match Command::new(&exe_path).args(&args).output() {
            Ok(o) => o,
            Err(err) => {
                eprintln!("Failed to run plugin {}: {}", exe_path.display(), err);
                return;
            }
        };
    }

    let stdout_bytes = output.stdout;

    let stdout_str = String::from_utf8_lossy(&stdout_bytes);
    send_combo(&stdout_str);
}
