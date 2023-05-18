use std::fs::read_to_string;

pub enum Status {
    Charging,
    Discharging
}

pub fn get_battery_data() -> Result<(String, Status), std::io::Error>{
     let data = read_to_string("/sys/class/power_supply/BAT1/capacity")?;
     let status = match read_to_string("/sys/class/power_supply/BAT1/status")?.as_str() {
         "Discharging" => Status::Discharging,
         "Charging" => Status::Charging,
         _ => Status::Charging
     };
     Ok((data, status))
}
