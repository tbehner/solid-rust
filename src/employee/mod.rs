pub struct Employee {
    name: String,
    address: String,
}

impl Employee {
    pub fn new(name: &str, address: &str) -> Employee {
        return Employee{
            name: String::from(name),
            address: String::from(address),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_address(&self) -> &str {
        self.address.as_ref()
    }
}
