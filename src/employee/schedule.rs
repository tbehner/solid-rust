use std::string::String;

pub trait PaymentSchedule {
    fn when_do_i_get_paid(&self) -> String;
}

pub struct WeeklySchedule{}
pub struct BiWeeklySchedule{}
pub struct MonthlySchedule{}

impl MonthlySchedule {
    pub fn new() -> MonthlySchedule {
        MonthlySchedule{}
    }
}

impl WeeklySchedule{
    pub fn new() -> BiWeeklySchedule{
        BiWeeklySchedule{}
    }
}

impl BiWeeklySchedule{
    pub fn new() -> BiWeeklySchedule{
        BiWeeklySchedule{}
    }
}

impl PaymentSchedule for WeeklySchedule{
    fn when_do_i_get_paid(&self) -> String{
        String::from("every week")
    }
}

impl PaymentSchedule for BiWeeklySchedule{
    fn when_do_i_get_paid(&self) -> String{
        String::from("every two weeks")
    }
}

impl PaymentSchedule for MonthlySchedule{
    fn when_do_i_get_paid(&self) -> String{
        String::from("every four weeks")
    }
}
