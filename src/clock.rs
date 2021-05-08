use chrono::{Date, Utc};

pub trait Clock {
    fn now(&self) -> Date<Utc>;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Date<Utc> {
        Utc::today()
    }
}
