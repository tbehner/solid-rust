use std::any::Any;

pub trait PaymentMethod {
    fn as_any(&self) -> &dyn Any;
}

pub struct HoldMethod {}

impl HoldMethod {
    pub fn new() -> HoldMethod {
        HoldMethod{}
    }
}

impl PaymentMethod for HoldMethod{
    fn as_any(&self) -> &dyn Any {
        self
    }
}
