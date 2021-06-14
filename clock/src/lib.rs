use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.hours;
        let minutes = self.minutes;

        return write!(f, "{}:{}", format!("{:0width$}", hours, width=2), format!("{:0width$}", minutes, width=2));
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let add_to_hours = (minutes as f32 / 60.0).floor() as i32;
        let correct_minutes = minutes % 60;
        let mut our_hours = hours + add_to_hours;
        our_hours = our_hours % 24;

        return Clock {
            hours: our_hours,
            minutes: correct_minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let self_minutes = self.minutes + minutes;

        return Clock::new(self.hours, self_minutes);
    }
}
