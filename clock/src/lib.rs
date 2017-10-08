use std::cmp;
use std::fmt;
use std::fmt::Display;

static MIN_TO_HOUR: i32 = 60;
static ONE_DAY: i32 = 60 * 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    pub hour: u32,
    pub minute: u32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl Clock {
    pub fn new(hour: i32, minute: i32) -> Clock {
        let total_min = hour * MIN_TO_HOUR + minute;
        return Clock::get_time(total_min)
    }

    fn get_time(total_min: i32) -> Clock {
        if total_min % ONE_DAY == 0 { return Clock { hour: 0, minute: 0 } }
        match total_min.cmp(&0) {
            cmp::Ordering::Equal => Clock { hour: 0, minute: 0 },
            cmp::Ordering::Greater => {
                if total_min > ONE_DAY {
                    return Clock::get_time(total_min - ONE_DAY)
                } else {
                    let hour = (total_min / MIN_TO_HOUR) as u32;
                    let minute = (total_min % MIN_TO_HOUR) as u32;
                    return Clock { hour: hour, minute: minute }
                }
            },
            cmp::Ordering::Less => {
                return Clock::get_time(total_min + ONE_DAY)
            },
        }
    }

    pub fn add_minutes(self, minute: i32) -> Clock {
        let total_min = (self.hour as i32) * MIN_TO_HOUR + (self.minute as i32) + minute;
        return Clock::get_time(total_min)
    }
}
