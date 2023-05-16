use rust_fetch::battery::get_battery_level;

fn main() {
    println!("Battery: {}", get_battery_level().unwrap());
}
