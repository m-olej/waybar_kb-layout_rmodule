// Run 'hyprctl devices' command in through the terminal
// Read output and create a data structure holding all keyboard types registered
// return the active keymap where main == true

use ::std::process::Command;
use regex::Regex;
use std::{collections::HashMap, u8};

#[derive(Debug)]
struct Keyboard {
    id: String,
    keymap: String,
    main: bool,
}

fn retrieve_devices() -> Vec<u8> {
    let kb_data = Command::new("hyprctl")
        .arg("devices")
        .output()
        .expect("kb-layout find fail");

    kb_data.stdout
}

fn parse_data(devices: Vec<u8>) -> HashMap<String, Keyboard> {
    let mut dev_struct: HashMap<String, Keyboard> = HashMap::new();

    // regs
    let keyboard_re = Regex::new(r"Keyboard at ([a-f0-9]+):").unwrap();
    let keymap_re = Regex::new(r"active keymap: ([\w\s()]+)").unwrap();
    let main_re = Regex::new(r"main: (yes|no)").unwrap();

    if let Ok(devices_str) = std::str::from_utf8(&devices) {
        let mut curr_id = None;
        for line in devices_str.lines() {
            if let Some(k_id) = keyboard_re.captures(line) {
                curr_id = Some(k_id[1].to_string());
            } else if let Some(k_att) = keymap_re.captures(line) {
                if let Some(ref id) = curr_id {
                    let keyboard = dev_struct.entry(id.clone()).or_insert(Keyboard {
                        id: id.clone(),
                        keymap: Some(k_att[1].to_string()).unwrap(),
                        main: false,
                    });
                    keyboard.keymap = Some(k_att[1].to_string()).unwrap();
                }
            } else if let Some(k_main) = main_re.captures(line) {
                if let Some(ref id) = curr_id {
                    let keyboard = dev_struct.entry(id.clone()).or_insert(Keyboard {
                        id: id.clone(),
                        keymap: "".to_string(),
                        main: &k_main[1] == "yes",
                    });
                    keyboard.main = &k_main[1] == "yes";
                }
            }
        }
    }

    dev_struct
}

fn get_active_kb(dev_struct: HashMap<String, Keyboard>) -> String {
    if let Some((_, keyboard)) = dev_struct.iter().find(|(_, kb)| kb.main) {
        keyboard.keymap.clone()
    } else {
        "Unknown".to_string()
    }
}

fn main() {
    let devices: Vec<u8> = retrieve_devices();

    let dev_struct: HashMap<String, Keyboard> = parse_data(devices);

    let active_keymap = get_active_kb(dev_struct);

    println!("{}", active_keymap);
}
