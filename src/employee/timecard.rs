use chrono::prelude::*;

#[derive(Copy, Clone)]
pub struct TimeCard {
    hours: f32, 
    date: Date<Local>,
}

impl TimeCard {
    pub fn new(date: Date<Local>, hours: f32) -> TimeCard {
        TimeCard{
            hours: hours,
            date: date,
        }
    }

    pub fn get_hours(&self) -> f32 {
        self.hours
    }
}
