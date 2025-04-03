use std::fs;
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    aimbot_enabled: bool,
    esp_enabled: bool,
    no_recoil_enabled: bool,
    speed_hack_enabled: bool,
}

fn main() {
    let config = load_config();
    if config.aimbot_enabled {
        enable_aimbot();
    }
    if config.esp_enabled {
        enable_esp();
    }
    if config.no_recoil_enabled {
        enable_no_recoil();
    }
    if config.speed_hack_enabled {
        enable_speed_hack();
    }
    launch_game();
}

fn load_config() -> Config {
    let config_data = fs::read_to_string("config.json").expect("Unable to read config file");
    serde_json::from_str(&config_data).expect("Unable to parse config")
}

fn enable_aimbot() {
    // Aimbot logic here
}

fn enable_esp() {
    // ESP logic here
}

fn enable_no_recoil() {
    // No recoil logic here
}

fn enable_speed_hack() {
    // Speed hack logic here
}

fn launch_game() {
    Command::new("C:\\Path\\To\\Warzone.exe").spawn().expect("Failed to launch game");
}