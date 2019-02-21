use failure::Error;
use std::collections::HashMap;
use std::cell::RefCell;


struct Employee {
    name: String,
}

impl Employee {
    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}


struct PayrollDatabase {
    employees: Option<HashMap<u32, Employee>>,
}


impl PayrollDatabase {
    fn new() -> PayrollDatabase {
        PayrollDatabase{employees: None}
    }

    fn init(&mut self){
        match self.employees {
            None => {
                self.employees = Some(HashMap::new());
            },
            Some(_) => {},
        }
    }

    fn add_employee(&mut self, empid: u32, employee: Employee){
        match self.employees {
            Some(ref mut db) => db.insert(empid, employee),
            None => panic!("Use of uninitialized database!"),
        };
    }

    fn get_employee(&self, empid: u32) -> &Employee {
        match self.employees {
            Some(ref db) => db.get(&empid).unwrap(),
            None => panic!("Use of uninitialized database!"),
        }
    }
}


thread_local! {
    static GLOBAL_PAYROLL_DB: RefCell<PayrollDatabase> = RefCell::new(PayrollDatabase::new());
}


trait Transaction{
    fn execute(&self) -> Result<(),Error>;
}


trait AddEmployeeTransaction: Transaction {
}


struct AddSalariedEmployee{
    its_empid: u32,
    its_address: String,
    its_name: String,
    its_salary: f32,
}


impl AddSalariedEmployee{
    fn new(empid: u32, name: &str, address: &str, salary: f32) -> AddSalariedEmployee {
        AddSalariedEmployee{
            its_empid: empid,
            its_name: String::from(name),
            its_address: String::from(address),
            its_salary: salary,
        }
    }
}


impl Transaction for AddSalariedEmployee {
    fn execute(&self) -> Result<(), Error>{
        let employee = Employee{ name: self.its_name.clone() };

        GLOBAL_PAYROLL_DB.with(|db| {
            db.borrow_mut().init();
            db.borrow_mut().add_employee(self.its_empid, employee)
        });

        Ok(())
    }
}


impl AddEmployeeTransaction for AddSalariedEmployee{
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_employee(){
        let emp_id = 1;
        let t = AddSalariedEmployee::new(emp_id, "Bob", "Home", 1000.00);
        assert!(t.execute().is_ok(), "Could not add a salaried employee!");

        GLOBAL_PAYROLL_DB.with(|c| {
            let db = c.borrow();
            let employee = db.get_employee(emp_id);
            assert_eq!("Bob", employee.get_name());
        });
    }
}
