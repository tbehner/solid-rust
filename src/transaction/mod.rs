use failure::Error;
use crate::employee::Employee;
use crate::database::PayrollDatabase;
use crate::employee::{PaymentSchedule, MonthlySchedule, BiWeeklySchedule, WeeklySchedule};
use crate::employee::{PaymentClassification, SalariedClassification, HourlyClassification};
use crate::employee::{PaymentMethod, HoldMethod};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static GLOBAL_PAYROLL_DB: RefCell<PayrollDatabase> = RefCell::new(PayrollDatabase::new());
}



trait Transaction{
    fn execute(&self) -> Result<(),Error>;
}


trait AddEmployeeTransaction: Transaction {
    fn employee_id(&self) -> u32;
    fn employee_name(&self) -> String;
    fn employee_address(&self) -> String;
    fn employee_salary(&self) -> f32;

    fn get_classification(&self) -> Rc<dyn PaymentClassification>;
    fn get_schedule(&self) -> Rc<dyn PaymentSchedule>;
    fn get_payment_method(&self) -> Rc<dyn PaymentMethod>;
}


impl<T: AddEmployeeTransaction> Transaction for T {
    fn execute(&self) -> Result<(), Error>{
        let employee = Employee::new(
            &self.employee_name(),
            &self.employee_address(),
            self.get_classification(),
            self.get_schedule(),
            self.get_payment_method(),
            );

        GLOBAL_PAYROLL_DB.with(|db| {
            db.borrow_mut().init();
            db.borrow_mut().add_employee(self.employee_id(), employee)
        });

        Ok(())
    }
}


struct AddSalariedEmployee{
    its_empid: u32,
    its_address: String,
    its_name: String,
    its_salary: f32,
    its_schedule: Rc<dyn PaymentSchedule>
}


impl AddSalariedEmployee{
    fn new(empid: u32, name: &str, address: &str, salary: f32, schedule: Rc<dyn PaymentSchedule>) -> AddSalariedEmployee {
        AddSalariedEmployee{
            its_empid: empid,
            its_name: String::from(name),
            its_address: String::from(address),
            its_salary: salary,
            its_schedule: schedule,
        }
    }
}


impl AddEmployeeTransaction for AddSalariedEmployee{
    fn employee_id(&self) -> u32 {
        self.its_empid
    }

    fn employee_name(&self) -> String {
        self.its_name.clone()
    }

    fn employee_address(&self) -> String {
        self.its_address.clone()
    }

    fn employee_salary(&self) -> f32 {
        self.its_salary
    }

    fn get_schedule(&self) -> Rc<dyn PaymentSchedule> {
        self.its_schedule.clone()
    }

    fn get_classification(&self) -> Rc<dyn PaymentClassification> {
        Rc::new(SalariedClassification::new(self.employee_salary()))
    }

    fn get_payment_method(&self) -> Rc<dyn PaymentMethod> {
        Rc::new(HoldMethod::new())
    }
}


struct AddHourlyEmployee{
    its_empid: u32,
    its_address: String,
    its_name: String,
    its_hourly_rate: f32,
    its_schedule: Rc<dyn PaymentSchedule>
}


impl AddHourlyEmployee{
    fn new(empid: u32, name: &str, address: &str, salary: f32, schedule: Rc<dyn PaymentSchedule>) -> AddHourlyEmployee {
        AddHourlyEmployee{
            its_empid: empid,
            its_name: String::from(name),
            its_address: String::from(address),
            its_hourly_rate: salary,
            its_schedule: schedule,
        }
    }
}


impl AddEmployeeTransaction for AddHourlyEmployee{
    fn employee_id(&self) -> u32 {
        self.its_empid
    }

    fn employee_name(&self) -> String {
        self.its_name.clone()
    }

    fn employee_address(&self) -> String {
        self.its_address.clone()
    }

    fn employee_salary(&self) -> f32 {
        self.its_hourly_rate
    }

    fn get_schedule(&self) -> Rc<dyn PaymentSchedule> {
        self.its_schedule.clone()
    }

    fn get_classification(&self) -> Rc<dyn PaymentClassification> {
        Rc::new(HourlyClassification::new(self.employee_salary()))
    }

    fn get_payment_method(&self) -> Rc<dyn PaymentMethod> {
        Rc::new(HoldMethod::new())
    }



}


#[cfg(test)]
mod tests {
    use super::*;

    fn add_salaried_employee(id: u32, schedule: Rc<dyn PaymentSchedule>){
        let t = AddSalariedEmployee::new(id, "Bob", "Home", 1000.00, schedule);
        assert!(t.execute().is_ok(), "Could not add a salaried employee!");
    }

    #[test]
    fn employee_has_correct_stats(){
        let emp_id = 0;
        add_salaried_employee(emp_id, Rc::new(MonthlySchedule::new()));

        GLOBAL_PAYROLL_DB.with(|connection| {
            let db = connection.borrow();
            let employee = db.get_employee(emp_id);
            assert_eq!("Bob", employee.get_name());
            assert_eq!("Home", employee.get_address());
            let classification = employee.get_classification();
            let sc = classification.as_any().downcast_ref::<SalariedClassification>();
            assert!(sc.is_some());
            assert_eq!(sc.unwrap().get_salary(), 1000.00);
            assert!(employee.get_method().as_any().downcast_ref::<HoldMethod>().is_some());
        });
    }

    #[test]
    fn monthly_employee_is_created(){
        let emp_id = 1;
        add_salaried_employee(emp_id, Rc::new(MonthlySchedule::new()));
        GLOBAL_PAYROLL_DB.with(|connection| {
            let db = connection.borrow();
            let employee = db.get_employee(emp_id);
            assert_eq!(String::from("every four weeks"), employee.get_schedule().when_do_i_get_paid());
        });
    }

    #[test]
    fn biweekly_employee_is_created(){
        let emp_id = 2;
        add_salaried_employee(emp_id, Rc::new(BiWeeklySchedule::new()));
        GLOBAL_PAYROLL_DB.with(|connection| {
            let db = connection.borrow();
            let employee = db.get_employee(emp_id);
            assert_eq!(String::from("every two weeks"), employee.get_schedule().when_do_i_get_paid());
        });
    }

    #[test]
    fn time_card_transaction(){
        let emp_id = 1;
        let schedule = Rc::new(WeeklySchedule::new());
        let add_transaction = AddHourlyEmployee::new(emp_id, "Bill", "Home", 15.25, schedule);
        assert!(add_transaction.execute().is_ok(), "Could not add hourle employee!");
    }
}
