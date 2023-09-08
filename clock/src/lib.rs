use core::fmt;

static HOURS_IN_A_DAY: i32 = 24;
static MINUTES_IN_AN_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours_delta, minutes) = (
            minutes.div_euclid(MINUTES_IN_AN_HOUR),
            minutes.rem_euclid(MINUTES_IN_AN_HOUR),
        );

        let hours = (hours + hours_delta).rem_euclid(HOURS_IN_A_DAY);

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02.2}:{:02.2}", self.hours, self.minutes)
    }
}
