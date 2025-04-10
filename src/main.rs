use std::process::{Command};

const BATTERY_PERCENT: &str = "/sys/class/power_supply/BAT0/capacity";
const BATTERY_STATUS: &str = "/sys/class/power_supply/BAT0/status";

fn supported_de() -> bool{
    let de: &str = &std::env::var("DESKTOP_SESSION").unwrap_or_default();
    match de {
        "hyprland" | "i3wm" | "bspwm" => true,
        _ => false
    }
}

fn get_battery_percent() -> usize{
    let output = Command::new("cat").arg(BATTERY_PERCENT).output();

    match output {
        Ok(output) => {
            let battery_value = String::from_utf8_lossy(&output.stdout);
            battery_value.trim().parse().unwrap_or(100)
        }

        Err(_) => {
            100
        }
    }
}

fn get_charging_status() -> String{
    let output = Command::new("cat").arg(BATTERY_STATUS).output();

    match output {
        Ok(output) => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }

        Err(_) => {
            "Discharging".to_string()
        }
    }
}

fn main() {
    if supported_de(){
        loop{
            println!("{}", get_battery_percent()); 
            std::thread::sleep(std::time::Duration::from_secs(5));
            if get_battery_percent() < 20 && Command::new("powerprofilectl")
                .arg("get")
                .output()
                .map(|output|{
                    String::from_utf8_lossy(&output.stdout).trim() != "power-saver"
                }).unwrap_or(false){
                    Command::new("powerprofilectl").arg("set").arg("power-saver");
                    println!("set to power-saver");
            }
            if get_charging_status() == "Charging" && Command::new("powerprofilectl")
                .arg("get")
                .output()
                .map(|output|{
                    String::from_utf8_lossy(&output.stdout).trim() != "performance"
                }).unwrap_or(false){
                    Command::new("powerprofilectl").arg("set").arg("performance");
                    println!("set to performance");
            }
        }
    }
    else{
        println!("DE is not supported, can't run a daemon");
        return;
    }
}
