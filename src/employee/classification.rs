use std::any::Any;

pub trait PaymentClassification{
    fn as_any(&self) -> &dyn Any;
}

pub struct SalariedClassification{
    value: f32,
}

impl SalariedClassification {
    pub fn new(value: f32) -> SalariedClassification {
        SalariedClassification{value: value}
    }
}

impl PaymentClassification for SalariedClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }
}


