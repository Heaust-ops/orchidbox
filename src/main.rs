mod config;
mod simulation;
use active_win_pos_rs::get_active_window;
use gilrs::{Button, Event, Gilrs};
use std::{fs::read_to_string, path::Path, process::Command};

use config::Config;

use crate::simulation::{
    move_mouse_delta, scroll_mouse, send_combo, send_left_click, send_middle_click,
    send_right_click,
};

fn run_plugin_and_send_combo(input: &str) {
    // Split on whitespace. If you need shell-style quoting support, see note below.
    let mut parts = input.split("+");

    let bin_token = parts.next().expect("failed to parse plugin command");
    let binname = &bin_token[1..]; // strip leading '@'

    let args: Vec<&str> = parts.collect();

    // Build executable path: ./plugins/<binname>
    let exe_path = Path::new("./plugins").join(binname);

    if !exe_path.exists() {
        return;
    }

    // Run and capture stdout (waits for exit)
    let output = match Command::new(&exe_path).args(&args).output() {
        Ok(o) => o,
        Err(err) => {
            eprintln!("Failed to run plugin {}: {}", exe_path.display(), err);
            return;
        }
    };

    let stdout_bytes = output.stdout;

    let stdout_str = String::from_utf8_lossy(&stdout_bytes);
    send_combo(&stdout_str);
}

fn get_gamepad_key(button: Button) -> &'static str {
    use gilrs::Button::*;

    match button {
        // Face buttons
        South => "a",
        East => "b",
        West => "x",
        North => "y",

        // D-pad
        DPadUp => "up",
        DPadDown => "down",
        DPadLeft => "left",
        DPadRight => "right",

        // Triggers / bumpers
        LeftTrigger2 => "lb",
        LeftTrigger => "lt",
        RightTrigger2 => "rb",
        RightTrigger => "rt",

        // Sticks
        LeftThumb => "ls",
        RightThumb => "rs",

        // Menu buttons
        Start => "start",
        Select => "select",
        Mode => "guide",

        // Misc / unsupported
        C | Z | Unknown => "unknown",
    }
}

fn modulate_stick_sensitivity_for_mouse(lx: f32, ly: f32) -> (f64, f64) {
    let deadzone = 0.;

    let gamma = 3.;

    let max_sensitivity = 1.0;

    let mut dx = 0.0f64;
    let mut dy = 0.0f64;

    if lx.abs() > deadzone {
        let norm = (lx.abs() - deadzone) / (1.0 - deadzone);
        let curved = norm.powf(gamma);
        dx = lx.signum() as f64 * curved as f64 * max_sensitivity;
    }

    if ly.abs() > deadzone {
        let norm = (ly.abs() - deadzone) / (1.0 - deadzone);
        let curved = norm.powf(gamma);
        dy = -ly.signum() as f64 * curved as f64 * max_sensitivity;
    }

    (dx, dy)
}

struct MouseAccumulator {
    x: f32,
    y: f32,
    sx: f32,
    sy: f32,
}

fn main() {
    let config_content = read_to_string("./config.css").expect("failed to read config");
    let cfg = Config::load_from(&config_content);
    cfg.print();
    println!("===");
    println!("{:#?}", cfg.query(".does_not_exist".to_string()));

    let mut gampad_key_stack: Vec<String> = vec![];

    let mut gilrs = Gilrs::new().unwrap();

    let mut is_mouse_mode: bool = false;

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    let mut mouse_acc = MouseAccumulator {
        x: 0.,
        y: 0.,
        sx: 0.,
        sy: 0.,
    };

    let mut active_app_query = cfg.query("uninitialized".to_string());

    let mut prev_window = get_active_window().expect("failed to get active window");
    let mut current_window = get_active_window().expect("failed to get active window");

    loop {
        // Examine new events
        while let Some(Event { id, event, .. }) = gilrs.next_event() {
            active_gamepad = Some(id);

            if let Ok(win) = get_active_window()
                && win.window_id != prev_window.window_id
            {
                prev_window = current_window;
                current_window = win;
                active_app_query = cfg.query(format!(".{}", current_window.app_name));
            }

            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    gampad_key_stack.push(get_gamepad_key(button).to_string());

                    if let Some(val) = active_app_query.get(&gampad_key_stack.join("-")) {
                        match val.as_str() {
                            "mouse" => {
                                is_mouse_mode = true;
                            }
                            "click" | "leftclick" => {
                                send_left_click(false);
                            }
                            "rightclick" => {
                                send_right_click(false);
                            }
                            "middleclick" => {
                                send_middle_click(false);
                            }
                            _ => {}
                        }
                    } else {
                        is_mouse_mode = false;
                    }
                }
                gilrs::EventType::ButtonReleased(button, _) => {
                    if let Some(val) = active_app_query.get(&gampad_key_stack.join("-")) {
                        match val.as_str() {
                            "click" | "leftclick" => {
                                send_left_click(true);
                            }
                            "rightclick" => {
                                send_right_click(true);
                            }
                            "middleclick" => {
                                send_middle_click(true);
                            }
                            _ => {}
                        }

                        println!("match: {:?}", val);
                        if val.starts_with("@") {
                            let _ = run_plugin_and_send_combo(val);
                        } else {
                            send_combo(val);
                        }
                    }

                    let button_string = get_gamepad_key(button).to_string();
                    if let Some(pos) = gampad_key_stack.iter().position(|x| *x == button_string) {
                        gampad_key_stack.remove(pos);
                    };

                    if let Some(val) = active_app_query.get(&gampad_key_stack.join("-")) {
                        if val == "mouse" {
                            is_mouse_mode = true;
                        } else {
                            is_mouse_mode = false;
                        }
                    } else {
                        is_mouse_mode = false;
                    }
                }
                _ => {}
            }
        }

        if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if !is_mouse_mode {
                continue;
            };

            mouse_acc.x += gamepad.value(gilrs::Axis::LeftStickX);
            mouse_acc.y += gamepad.value(gilrs::Axis::LeftStickY);

            let mut m_coords = (0., 0.);
            if mouse_acc.x.abs() > 1. {
                m_coords.0 = modulate_stick_sensitivity_for_mouse(mouse_acc.x.into(), 0.).0;
                mouse_acc.x = 0.;
            }
            if mouse_acc.y.abs() > 1. {
                m_coords.1 = modulate_stick_sensitivity_for_mouse(0., mouse_acc.y.into()).1;
                mouse_acc.y = 0.;
            }

            move_mouse_delta(m_coords.0, m_coords.1);

            let scroll_sensitivity = 0.001 * 9.;

            mouse_acc.sx += gamepad.value(gilrs::Axis::RightStickX) * scroll_sensitivity;
            mouse_acc.sy += gamepad.value(gilrs::Axis::RightStickY) * scroll_sensitivity;

            let mut m_scroll_coords = (0., 0.);
            if mouse_acc.sx.abs() > 1. {
                m_scroll_coords.0 = modulate_stick_sensitivity_for_mouse(mouse_acc.sx.into(), 0.).0;
                mouse_acc.sx = 0.;
            }
            if mouse_acc.sy.abs() > 1. {
                m_scroll_coords.1 = modulate_stick_sensitivity_for_mouse(0., mouse_acc.sy.into()).1;
                mouse_acc.sy = 0.;
            }

            scroll_mouse((m_scroll_coords.0) as i64, -(m_scroll_coords.1) as i64);
        }
    }
}
