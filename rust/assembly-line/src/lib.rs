pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as u64;
    let base_production_per_hour: u64 = 221;
    let production_per_hour = (speed * base_production_per_hour) as f64;

    if speed <= 4 {
        production_per_hour
    } else if speed <= 8 {
        let success_rate = 0.9;
        production_per_hour * success_rate
    } else {
        let success_rate = 0.77;
        production_per_hour * success_rate
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
