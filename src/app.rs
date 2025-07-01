use std::{fs, process::Command};

use crate::lang::{get_langs, Lang};


pub enum Screen {
    LanguageSelection,
    WifiSelection,
    DiskSelection,
    Partitioning,
    Efipart,
    UserSetup,
    Installing,
    Complete,
}

#[derive(Clone)]
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
    pub disk_list: Vec<(String, Vec<String>)>,
    pub disk_tom_efi: Option<String>,
    pub disk_tom: String,
    pub disk: usize,
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
            disk_list: Vec::new(),
            disk_tom_efi: None,
            disk_tom: String::new(),
            disk: 0,
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


    pub fn wifi_check(&mut self) -> bool {
        let output = Command::new("ping")
            .args(["-c", "1", "8.8.8.8"])
            .output()
            .expect("failed to ping");
        let wifi_check_text = String::from_utf8_lossy(&output.stdout);

        match wifi_check_text.find("ping: connect: Network is unreachable") {
            Some(_) => true,
            None => true,
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
    pub fn time_sync(&mut self) {
        let output = Command::new("timedatectl").output().expect("failed to run timedatectl");
        let time_sync_check = String::from_utf8_lossy(&output.stdout);
        let time_sync_check = time_sync_check.find("System clock synchronized: yes");

        if let None = time_sync_check {
            fs::write("/etc/systemd/timesyncd.conf", "
[Time]
NTP=time.cloudflare.com time.google.com
FallbackNTP=time.cloudflare.com time.google.com 0.arch.pool.ntp.org 1.arch.pool.ntp.org 2.arch.pool.ntp.org 3.arch.pool.ntp.org
").expect("Timesync conf save error");
            self.logs.push("Timesync save conf".to_string());

            Command::new("systemctl")
                .args(["restart", "systemd-timesyncd.service"])
                .output()
                .expect("failed to restart timesyncd");
            self.logs.push("Timesyncd to restart".to_string());
        }
    }


    pub fn set_disk_list(&mut self) {
        if self.disk_list.len() == 0 {
            let output = Command::new("lsblk")
                .output()
                .expect("failed to run lsblk");
            let disk_list_text = String::from_utf8_lossy(&output.stdout);

            let mut disk: usize = 0;

            for mut disk_text in disk_list_text.lines().skip(1) {
                let mut part_check = false;

                if let Some((_, after)) = disk_text.split_once("─") {
                    disk_text = after;
                    part_check = true;
                }

                if let Some((before, _)) = disk_text.split_once(" ") {
                    if !part_check {
                        disk += 1;
                        self.disk_list.push((before.to_string(), Vec::new()));
                        self.logs.push("Disk detect: ".to_string() + before);
                    } else {
                        self.disk_list.get_mut(disk - 1)
                            .expect("fatal of disk list")
                            .1.push(before.to_string());
                        self.logs.push("Part detect: ".to_string() + before);
                    }
                }
            }
        }

        if self.disk_list.len() == 0 {
            panic!("failed to get disk list")
        }
    }

    pub fn efi_check(&mut self) -> bool {
        match fs::read("/sys/firmware/efi/fw_platform_size") {
            Ok(_) => {
                self.logs.push("Efi is true".to_string());
                true
            }
            Err(_) => {
                self.logs.push("Efi is false".to_string());
                true
            }
        }
    }
}