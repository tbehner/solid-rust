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

    pub fn get_salary(&self) -> f32 {
        self.value
    }
}

impl PaymentClassification for SalariedClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct HourlyClassification {
    value: f32,
}

impl HourlyClassification {
    pub fn new(value: f32) -> HourlyClassification {
        HourlyClassification{value: value}
    }

    pub fn get_salary(&self) -> f32 {
        self.value
    }
}

impl PaymentClassification for HourlyClassification {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
