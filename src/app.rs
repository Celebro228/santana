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
    pub install_list: Vec<(String, Vec<String>)>,
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
            install_list: Vec::new(),
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
        Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg("8.8.8.8")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
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

    pub fn mount_and_format(&mut self) {
        Command::new("mkfs.ext4")
            .arg("/dev/".to_string() + &self.disk_tom)
            .output()
            .expect(&("Failed to format ext4: ".to_string() + &self.disk_tom));
        self.logs.push("Format to ext4: ".to_string() + &self.disk_tom);
        Command::new("mount")
            .args(["/dev/".to_string() + &self.disk_tom, "/mnt".to_string()])
            .output()
            .expect(&("Failed to mount ext4: ".to_string() + &self.disk_tom));
        self.logs.push("Mount to /mnt: /dev/".to_string() + &self.disk_tom);


        if let Some(part) = &self.disk_tom_efi {
            Command::new("mkfs.vfat")
                .arg("/dev/".to_string() + part)
                .output()
                .expect(&format!("failed to format part {} for efi", part));
            self.logs.push("Format to vfat: ".to_string() + &part);
            Command::new("mkdir")
                .args(["-p", "/mnt/boot/efi"])
                .output()
                .expect("failed create efi path");
            Command::new("mount")
                .args(["/dev/".to_string() + &part, "/mnt/boot/efi".to_string()])
                .output()
                .expect(&("Failed to mount ext4: ".to_string() + &self.disk_tom));
            self.logs.push("Mount to /mnt/boot/efi: /dev/".to_string() + &self.disk_tom);
        }
    }

    pub fn set_install_list(&mut self) {
        self.install_list.push(("Linux".to_string(), vec![
            "base".to_string(),
            "base-devel".to_string(),
            "linux".to_string(),
            "linux-firmware".to_string(),
        ]));
        self.install_list.push(("Display drivers".to_string(), vec![
            "wayland".to_string(),
            "xorg-xwayland".to_string(),
            "brightnessctl".to_string(),
        ]));
        self.install_list.push(("Audio drivers".to_string(), vec![
            "pipewire".to_string(),
            "pipewire-alsa".to_string(),
            "pipewire-jack".to_string(),
            "pipewire-pulse".to_string(),
            "gst-plugin-pipewire".to_string(),
            "libpulse".to_string(),
            "wireplumber".to_string(),
        ]));


        let output = Command::new("lspci")
            .arg("-mm")
            .output()
            .expect("failed to run lspci");
        let gpus_text = String::from_utf8_lossy(&output.stdout).to_lowercase();

        let mut nvidia_gpu = false;
        let mut amd_gpu = false;
        let mut intel_gpu = false;
        let mut intel_audio = false;

        for gpu in gpus_text.lines() {
            if gpu.contains("vga") || gpu.contains("3d controller") {
                if gpu.contains("nvidia") {
                    nvidia_gpu = true;
                } else if gpu.contains("amd") || gpu.contains("ati") {
                    amd_gpu = true;
                } else if gpu.contains("intel") {
                    intel_gpu = true;
                }
            } else if gpu.contains("audio") {
                if gpu.contains("intel") {
                    intel_audio = true;
                }
            }
        }

        if nvidia_gpu {
            self.install_list.push(("Nvidia gpu drivers".to_string(), vec![
                "nvidia".to_string(),
                "nvidia-utils".to_string(),
                "nvidia-setting".to_string(),
            ]));
            self.logs.push("Nvidia gpu detect".to_string());
        }
        if amd_gpu {
            self.install_list.push(("Amd gpu drivers".to_string(), vec![
                "vulkan-radeon".to_string(),
            ]));
            self.logs.push("Amd gpu detect".to_string());
        }
        if intel_gpu {
            self.install_list.push(("Amd gpu drivers".to_string(), vec![
                "vulkan-intel".to_string(),
            ]));
            self.logs.push("Amd gpu detect".to_string());
        }
        if amd_gpu || intel_gpu {
            self.install_list.push(("Mesa drivers".to_string(), vec![
                "mesa".to_string(),
            ]));
        }
        if intel_audio {
            self.install_list.push(("Intel audio drivers".to_string(), vec![
                "sof-firmware".to_string(),
            ]));
            self.logs.push("Intel audio detect".to_string());
        }


        let output = fs::read_to_string("/proc/cpuinfo").unwrap_or_default().to_lowercase();
        let amd_cpu = output.contains("authenticamd");
        let intel_cpu = output.contains("genuineintel");

        if amd_cpu {
            self.install_list.push(("Amd cpu drivers".to_string(), vec![
                "amd-ucode".to_string(),
            ]));
            self.logs.push("Amd cpu detect".to_string());
        }
        if intel_cpu {
            self.install_list.push(("Intel cpu drivers".to_string(), vec![
                "intel-ucode".to_string(),
            ]));
            self.logs.push("Intel cpu detect".to_string());
        }


        self.install_list.push(("Working environment".to_string(), vec![
            "hyprland".to_string(),
            "sddm".to_string(),
            "grub".to_string(),
        ]));
        if let Some(_) = self.disk_tom_efi {
            self.install_list.push(("Efi boot loader".to_string(), vec![
                "efibootmgr".to_string(),
            ]));
            self.logs.push("Efi detect".to_string());
        }

        self.install_list.push(("Default apps".to_string(), vec![
            "nano".to_string(),
            "wget".to_string(),
            "sudo".to_string(),
            "networkmanager".to_string(),
        ]));
    }

    pub fn install(&mut self) {
        let install_package = self.install_list
            .get(self.select_num)
            .expect("Error select install list")
            .clone();

        let mut install_list = vec!["/mnt".to_string()];
        install_list.extend(install_package.1);

        Command::new("pacstrap")
            .args(install_list)
            .output()
            .expect(&("Failed install: ".to_string() + &install_package.0));
        self.logs.push("Install: ".to_string() + &install_package.0);
    }

    pub fn complite(&mut self) {
        
    }
}