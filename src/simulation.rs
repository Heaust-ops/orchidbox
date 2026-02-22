use mouse_position::mouse_position::Mouse;
use rdev::{Button, EventType, Key, simulate};
use std::{thread, time::Duration};

pub fn scroll_mouse(dx: i64, dy: i64) {
    simulate(&EventType::Wheel {
        delta_x: dx,
        delta_y: dy,
    })
    .unwrap();
}

pub fn move_mouse(x: f64, y: f64) {
    simulate(&EventType::MouseMove { x, y }).unwrap();
}

pub fn move_mouse_delta(dx: f64, dy: f64) {
    if let Mouse::Position { x, y } = Mouse::get_mouse_position() {
        let new_x = x as f64 + dx;
        let new_y = y as f64 + dy;
        move_mouse(new_x, new_y);
    }
}

pub fn send_left_click(is_release: bool) {
    if is_release {
        simulate(&EventType::ButtonRelease(Button::Left)).unwrap();
    } else {
        simulate(&EventType::ButtonPress(Button::Left)).unwrap();
    }
}

pub fn send_right_click(is_release: bool) {
    if is_release {
        simulate(&EventType::ButtonRelease(Button::Right)).unwrap();
    } else {
        simulate(&EventType::ButtonPress(Button::Right)).unwrap();
    }
}

pub fn send_middle_click(is_release: bool) {
    if is_release {
        simulate(&EventType::ButtonRelease(Button::Middle)).unwrap();
    } else {
        simulate(&EventType::ButtonPress(Button::Middle)).unwrap();
    }
}

pub fn send_combo(combo: &str) {
    let parts: Vec<&str> = combo.split('+').collect();

    let mut modifiers: Vec<Key> = Vec::new();
    let mut specials: Vec<Key> = Vec::new();
    let mut text = String::new();

    for p in parts {
        let token = p.to_lowercase();

        match token.as_str() {
            // modifiers
            "ctrl" => modifiers.push(Key::ControlLeft),
            "shift" => modifiers.push(Key::ShiftLeft),
            "meta" => modifiers.push(Key::MetaLeft),
            "alt" => modifiers.push(Key::Alt),

            // arrows
            "up" => specials.push(Key::UpArrow),
            "down" => specials.push(Key::DownArrow),
            "left" => specials.push(Key::LeftArrow),
            "right" => specials.push(Key::RightArrow),

            // other named keys
            "tab" => specials.push(Key::Tab),
            "enter" => specials.push(Key::Return),
            "esc" => specials.push(Key::Escape),
            "space" => send_char(' '),

            // mouse
            "mouse" | "click" | "leftclick" | "rightclick" | "middleclick" => {}

            // everything else = literal text
            _ => text.push_str(p),
        }
    }

    // hold modifiers
    for m in &modifiers {
        simulate(&EventType::KeyPress(*m)).unwrap();
    }

    // press special keys
    for k in &specials {
        simulate(&EventType::KeyPress(*k)).unwrap();
        simulate(&EventType::KeyRelease(*k)).unwrap();
    }

    // type characters
    for c in text.chars() {
        send_char(c);
    }

    // release modifiers
    for m in modifiers.iter().rev() {
        simulate(&EventType::KeyRelease(*m)).unwrap();
    }
}

fn send_char(c: char) {
    let (key, needs_shift) = char_to_key(c);

    if needs_shift {
        simulate(&EventType::KeyPress(Key::ShiftLeft)).unwrap();
    }

    simulate(&EventType::KeyPress(key)).unwrap();
    simulate(&EventType::KeyRelease(key)).unwrap();

    if needs_shift {
        simulate(&EventType::KeyRelease(Key::ShiftLeft)).unwrap();
    }

    thread::sleep(Duration::from_millis(5));
}

fn char_to_key(c: char) -> (Key, bool) {
    use Key::*;

    if c.is_ascii_uppercase() {
        return (
            match c.to_ascii_lowercase() {
                'a' => KeyA,
                'b' => KeyB,
                'c' => KeyC,
                'd' => KeyD,
                'e' => KeyE,
                'f' => KeyF,
                'g' => KeyG,
                'h' => KeyH,
                'i' => KeyI,
                'j' => KeyJ,
                'k' => KeyK,
                'l' => KeyL,
                'm' => KeyM,
                'n' => KeyN,
                'o' => KeyO,
                'p' => KeyP,
                'q' => KeyQ,
                'r' => KeyR,
                's' => KeyS,
                't' => KeyT,
                'u' => KeyU,
                'v' => KeyV,
                'w' => KeyW,
                'x' => KeyX,
                'y' => KeyY,
                'z' => KeyZ,
                _ => unreachable!(),
            },
            true,
        );
    }

    if c.is_ascii_lowercase() {
        return (
            match c {
                'a' => KeyA,
                'b' => KeyB,
                'c' => KeyC,
                'd' => KeyD,
                'e' => KeyE,
                'f' => KeyF,
                'g' => KeyG,
                'h' => KeyH,
                'i' => KeyI,
                'j' => KeyJ,
                'k' => KeyK,
                'l' => KeyL,
                'm' => KeyM,
                'n' => KeyN,
                'o' => KeyO,
                'p' => KeyP,
                'q' => KeyQ,
                'r' => KeyR,
                's' => KeyS,
                't' => KeyT,
                'u' => KeyU,
                'v' => KeyV,
                'w' => KeyW,
                'x' => KeyX,
                'y' => KeyY,
                'z' => KeyZ,
                _ => unreachable!(),
            },
            false,
        );
    }

    match c {
        '0' => (Num0, false),
        '1' => (Num1, false),
        '2' => (Num2, false),
        '3' => (Num3, false),
        '4' => (Num4, false),
        '5' => (Num5, false),
        '6' => (Num6, false),
        '7' => (Num7, false),
        '8' => (Num8, false),
        '9' => (Num9, false),

        '!' => (Num1, true),
        '@' => (Num2, true),
        '#' => (Num3, true),
        '$' => (Num4, true),
        '%' => (Num5, true),
        '^' => (Num6, true),
        '&' => (Num7, true),
        '*' => (Num8, true),
        '(' => (Num9, true),
        ')' => (Num0, true),

        ' ' => (Space, false),
        '\n' => (Return, false),
        '.' => (Dot, false),
        ',' => (Comma, false),
        '-' => (Minus, false),
        '=' => (Equal, false),
        '/' => (Slash, false),
        ';' => (SemiColon, false),
        '\'' => (Quote, false),

        '>' => (Dot, true),
        '<' => (Comma, true),
        '_' => (Minus, true),
        '+' => (Equal, true),
        '?' => (Slash, true),
        ':' => (SemiColon, true),
        '"' => (Quote, true),
        '{' => (LeftBracket, true),
        '}' => (RightBracket, true),
        '|' => (BackSlash, true),
        '~' => (BackQuote, true),

        _ => panic!("Unsupported character: {c:?}"),
    }
}
