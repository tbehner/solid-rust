use std::any::Any;
use crate::employee::TimeCard;
use chrono::prelude::*;

pub trait PaymentClassification{
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

pub struct SalariedClassification{
    value: f32,
}

impl SalariedClassification {
    pub fn new(value: f32) -> SalariedClassification {
        SalariedClassification{value: value}
    }

    pub fn get_salary(&self) -> f32 {
        self.value
    }
}

impl PaymentClassification for SalariedClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct HourlyClassification {
    value: f32,
    time_card: Option<TimeCard>,
}

impl HourlyClassification {
    pub fn new(value: f32) -> HourlyClassification {
        HourlyClassification{value: value, time_card: None}
    }

    pub fn get_salary(&self) -> f32 {
        self.value
    }

    pub fn add_time_card(&mut self, tc: TimeCard) {
        self.time_card = Some(tc)
    }

    pub fn get_time_card(&self, _date: &chrono::Date<Local>) -> Option<TimeCard> {
        match self.time_card {
            Some(ref tc) => Some(*tc),
            None => None,
        }
    }
}

impl PaymentClassification for HourlyClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
