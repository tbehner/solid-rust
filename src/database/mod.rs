use std::collections::HashMap;
use crate::employee::Employee;


pub struct PayrollDatabase {
    employees: Option<HashMap<u32, Employee>>,
}


impl PayrollDatabase {
    pub fn new() -> PayrollDatabase {
        PayrollDatabase{employees: None}
    }

    pub fn init(&mut self){
        match self.employees {
            None => {
                self.employees = Some(HashMap::new());
            },
            Some(_) => {},
        }
    }

    pub fn add_employee(&mut self, empid: u32, employee: Employee){
        match self.employees {
            Some(ref mut db) => db.insert(empid, employee),
            None => panic!("Use of uninitialized database!"),
        };
    }

    pub fn get_employee(&self, empid: u32) -> &Employee {
        match self.employees {
            Some(ref db) => db.get(&empid).unwrap(),
            None => panic!("Use of uninitialized database!"),
        }
    }
}
