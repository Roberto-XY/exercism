use std::fmt;

const HOURS_IN_DAY: i32 = 24;
const MINUTES_IN_HOUR: i32 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: Clock::calculate_hours(hours * MINUTES_IN_HOUR + minutes),
            minutes: Clock::calculate_minutes(minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(i32::from(self.hours), i32::from(self.minutes) + minutes)
    }

    fn calculate_hours(minutes: i32) -> u8 {
        let hour = (minutes / MINUTES_IN_HOUR) % HOURS_IN_DAY;

        if minutes % MINUTES_IN_HOUR < 0 {
            Clock::fix_negative_time(hour - 1, HOURS_IN_DAY) as u8
        } else if hour < 0 {
            Clock::fix_negative_time(hour, HOURS_IN_DAY) as u8
        } else {
            hour as u8
        }
    }

    fn calculate_minutes(minutes: i32) -> u8 {
        Clock::fix_negative_time(minutes % MINUTES_IN_HOUR, MINUTES_IN_HOUR) as u8
    }

    fn fix_negative_time(value: i32, base: i32) -> i32 {
        ((value + base) % base)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.hours, self.minutes) {
            (h @ 0..=9, m @ 0..=9) => write!(f, "0{}:0{}", h, m),
            (hh @ 10..=24, m @ 0..=9) => write!(f, "{}:0{}", hh, m),
            (h @ 0..=9, mm @ 10..=60) => write!(f, "0{}:{}", h, mm),
            (hh, mm) => write!(f, "{}:{}", hh, mm),
        }
    }
}
