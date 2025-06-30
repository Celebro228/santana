use std::process::Command;

use crate::lang::{get_langs, Lang};


pub enum Screen {
    LanguageSelection,
    WifiSelection,
    DiskSelection,
    Partitioning,
    UserSetup,
    Installing,
    Complete,
}

pub enum Editing {
    Name,
    Password,
}

pub struct Data {
    pub name: String,
    pub password: String,
}

impl Data {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            password: String::new(),
        }
    }
}


pub struct App {
    pub user: Data,
    pub wifi_device: String,
    pub wifi_list: Vec<String>,
    pub wifi: Data,
    pub language_list: Vec<Lang>,
    pub language: Lang,
    pub screen: Screen,
    pub editing: Option<Editing>,
    pub select_num: usize,
    pub debug_mode: bool,
    pub logs: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            user: Data::new(),
            wifi_device: String::new(),
            wifi_list: Vec::new(),
            wifi: Data::new(),
            language_list: get_langs(),
            language: Lang::en(),
            screen: Screen::LanguageSelection,
            editing: None,
            select_num: 0,
            debug_mode: false,
            logs: Vec::new(),
        }
    }


    pub fn set_wifi_list(&mut self) {
        if self.wifi_device == String::new() {
            let output = Command::new("iwctl")
                .args(["station", "list"])
                .output()
                .expect("failed to run iwctl");
            let devices_list_text = String::from_utf8_lossy(&output.stdout);

            for device_text in devices_list_text.lines().skip(3) {
                if let Some((_, after)) = device_text.split_once("  ") {
                    if let Some((before, _)) = after.split_once(" ") {
                        self.logs.push("Network device detect: ".to_string() + before);

                        self.wifi_device = before.to_string();
                        break;
                    }
                }
            }
        }

        if self.wifi_device == String::new() {
            panic!("failed to get network device")
        }

        let output = Command::new("iwctl")
            .args(["station", &self.wifi_device, "get-networks"])
            .output()
            .expect("failed to run iwctl");
        let wifi_list_text =  String::from_utf8_lossy(&output.stdout);

        self.wifi_list.clear();

        for wifi_text in wifi_list_text.lines().skip(3) {
            if let Some((_, after)) = wifi_text.split_once("      ") {
                if let Some((before, _)) = after.split_once(" ") {
                    self.logs.push("WiFi detect: ".to_string() + before);

                    self.wifi_list.push(before.to_string());
                }
            }
        }
    }


    pub fn wifi_connect(&mut self) {
        Command::new("iwctl")
            .args(["station", &self.wifi_device, "connect", &self.wifi.name, &self.wifi.password])
            .output()
            .expect("failed to run iwctl");

        self.logs.push("Wifi connect to ".to_string() + &self.wifi.name);
    }
}