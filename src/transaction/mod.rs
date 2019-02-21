use failure::Error;
use crate::employee::Employee;
use crate::database::PayrollDatabase;
use std::cell::RefCell;

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
        let employee = Employee::new(&self.its_name, &self.its_address);

        GLOBAL_PAYROLL_DB.with(|db| {
            db.borrow_mut().init();
            db.borrow_mut().add_employee(self.its_empid, employee)
        });

        Ok(())
    }
}


impl AddEmployeeTransaction for AddSalariedEmployee{}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_employee(){
        let emp_id = 1;
        let t = AddSalariedEmployee::new(emp_id, "Bob", "Home", 1000.00);
        assert!(t.execute().is_ok(), "Could not add a salaried employee!");

        GLOBAL_PAYROLL_DB.with(|connection| {
            let db = connection.borrow();
            let employee = db.get_employee(emp_id);
            assert_eq!("Bob", employee.get_name());
            assert_eq!("Home", employee.get_address());
        });
    }
}
