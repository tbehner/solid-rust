use std::any::Any;

pub trait PaymentSchedule {
    fn as_any(&self) -> &dyn Any;
}

pub struct MonthlySchedule{}

impl MonthlySchedule {
    pub fn new() -> MonthlySchedule {
        MonthlySchedule{}
    }
}

impl PaymentSchedule for MonthlySchedule{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
