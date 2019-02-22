mod classification;
mod schedule;

pub use self::classification::{PaymentClassification, SalariedClassification};
pub use self::schedule::{PaymentSchedule, MonthlySchedule};

use std::rc::Rc;

pub struct Employee {
    name: String,
    address: String,
    classification: Rc<dyn PaymentClassification>,
    schedule: Rc<dyn PaymentSchedule>,
}

impl Employee {
    pub fn new(name: &str, address: &str, 
               classification: Rc<dyn PaymentClassification>,
               schedule: Rc<dyn PaymentSchedule>
               ) -> Employee {
        return Employee{
            name: String::from(name),
            address: String::from(address),
            classification: classification,
            schedule: schedule,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_address(&self) -> &str {
        self.address.as_ref()
    }

    pub fn get_classification(&self) -> Rc<dyn PaymentClassification> {
        self.classification.clone()
    }

    pub fn get_schedule(&self) -> Rc<dyn PaymentSchedule> {
        self.schedule.clone()
    }
}
