use crate::types::*;

use rand::Rng;
use std::collections::HashMap;

pub fn get_keyboard_layout(layout: Option<&String>, shift: bool) -> KeyboardLayout {
    let default_layout = String::from("colemak");

    let layout = layout.unwrap_or(&default_layout);
    let mut layouts = HashMap::new();

    let f = |arr: &[&str; 10]| -> [String; 10] {
        arr.iter()
            .map(|&c| {
                if shift {
                    return match c {
                        "," => String::from("<"),
                        "." => String::from(">"),
                        "/" => String::from("?"),
                        ";" => String::from(":"),
                        _ => String::from(c).to_uppercase(),
                    };
                }
                String::from(c)
            })
            .collect::<Vec<String>>()
            .try_into()
            .unwrap()
    };

    let qwerty: [[String; 10]; 3] = [
        ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
        ["a", "s", "d", "f", "g", "h", "j", "k", "l", ";"],
        ["z", "x", "c", "v", "b", "n", "m", ",", ".", "/"],
    ]
    .iter()
    .map(f)
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();

    let colemak: [[String; 10]; 3] = [
        ["q", "w", "f", "p", "g", "j", "l", "u", "y", ";"],
        ["a", "r", "s", "t", "d", "h", "n", "e", "i", "o"],
        ["z", "x", "c", "v", "b", "k", "m", ",", ".", "/"],
    ]
    .iter()
    .map(f)
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();

    let dvorak: [[String; 10]; 3] = [
        ["'", ",", ".", "p", "y", "f", "g", "c", "r", "l"],
        ["a", "o", "e", "u", "i", "d", "h", "t", "n", "s"],
        [";", "q", "j", "k", "x", "b", "m", "w", "v", "z"],
    ]
    .iter()
    .map(f)
    .collect::<Vec<_>>()
    .try_into()
    .unwrap();

    layouts.insert(String::from("qwerty"), qwerty);
    layouts.insert(String::from("colemak"), colemak);
    layouts.insert(String::from("dvorak"), dvorak);

    if !layouts.contains_key(layout) {
        return KeyboardLayout {
            rows: layouts.get("qwerty").unwrap().clone(),
        };
    }

    return KeyboardLayout {
        rows: layouts.get(layout).unwrap().clone(),
    };
}

pub fn get_quote() -> Quote {
    let mut rng = rand::thread_rng();
    let idx: usize = rng.gen_range(0..QUOTES.len());
    let (name, text) = QUOTES[idx];

    Quote::new(String::from(name), String::from(text))
}
