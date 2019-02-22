mod classification;

pub use self::classification::{PaymentClassification, SalariedClassification};

use std::rc::Rc;

pub struct Employee {
    name: String,
    address: String,
    classification: Rc<dyn PaymentClassification>,
}

impl Employee {
    pub fn new(name: &str, address: &str, classification: Rc<dyn PaymentClassification>) -> Employee {
        return Employee{
            name: String::from(name),
            address: String::from(address),
            classification: classification,
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
}
