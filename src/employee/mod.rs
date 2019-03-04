mod classification;
mod schedule;
mod method;

pub use self::classification::{PaymentClassification, SalariedClassification};
pub use self::schedule::{PaymentSchedule, MonthlySchedule, BiWeeklySchedule};
pub use self::method::{PaymentMethod, HoldMethod};

use std::rc::Rc;

pub struct Employee {
    name: String,
    address: String,
    classification: Rc<dyn PaymentClassification>,
    schedule: Rc<dyn PaymentSchedule>,
    method: Rc<dyn PaymentMethod>,
}

impl Employee {
    pub fn new(name: &str, address: &str,
               classification: Rc<dyn PaymentClassification>,
               schedule: Rc<dyn PaymentSchedule>,
               method: Rc<dyn PaymentMethod>
               ) -> Employee {

        Employee{
            name: String::from(name),
            address: String::from(address),
            classification: classification,
            schedule: schedule,
            method: method,
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

    pub fn get_method(&self) -> Rc<dyn PaymentMethod> {
        self.method.clone()
    }
}
