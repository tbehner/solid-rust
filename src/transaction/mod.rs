use failure::Error;

use crate::employee::Employee;
use crate::database::PayrollDatabase;
use crate::employee::{PaymentSchedule, MonthlySchedule, BiWeeklySchedule, WeeklySchedule};
use crate::employee::{PaymentClassification, SalariedClassification, HourlyClassification};
use crate::employee::{PaymentMethod, HoldMethod};
use crate::employee::TimeCard;
use std::cell::RefCell;
use std::rc::Rc;
use chrono::prelude::*;

type EmployeeId = u32;

thread_local! {
    static GLOBAL_PAYROLL_DB: RefCell<PayrollDatabase> = RefCell::new(PayrollDatabase::new());
}



trait Transaction{
    fn execute(&self) -> Result<(),Error>;
}


trait AddEmployeeTransaction: Transaction {
    fn employee_id(&self) -> EmployeeId;
    fn employee_name(&self) -> String;
    fn employee_address(&self) -> String;
    fn employee_salary(&self) -> f32;

    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>>;
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
    its_empid: EmployeeId,
    its_address: String,
    its_name: String,
    its_salary: f32,
    its_schedule: Rc<dyn PaymentSchedule>
}


impl AddSalariedEmployee{
    fn new(empid: EmployeeId, name: &str, address: &str, salary: f32, schedule: Rc<dyn PaymentSchedule>) -> AddSalariedEmployee {
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
    fn employee_id(&self) -> EmployeeId {
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

    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(SalariedClassification::new(self.employee_salary())))
    }

    fn get_payment_method(&self) -> Rc<dyn PaymentMethod> {
        Rc::new(HoldMethod::new())
    }
}


struct AddHourlyEmployee{
    its_empid: EmployeeId,
    its_address: String,
    its_name: String,
    its_hourly_rate: f32,
    its_schedule: Rc<dyn PaymentSchedule>
}


impl AddHourlyEmployee{
    fn new(empid: EmployeeId, name: &str, address: &str, salary: f32, schedule: Rc<dyn PaymentSchedule>) -> AddHourlyEmployee {
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
    fn employee_id(&self) -> EmployeeId {
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

    fn get_classification(&self) -> Rc<RefCell<dyn PaymentClassification>> {
        Rc::new(RefCell::new(HourlyClassification::new(self.employee_salary())))
    }

    fn get_payment_method(&self) -> Rc<dyn PaymentMethod> {
        Rc::new(HoldMethod::new())
    }
}

struct TimeCardTransaction {
    // FIXME this will be some chrono type
    its_date: Date<Local>,
    // FIXME I don't even know what this is going to be
    its_hours: f32,
    its_empid: EmployeeId,
}

impl TimeCardTransaction {
    fn new(date: Date<Local>, hours: f32, empid: EmployeeId) -> TimeCardTransaction {
        TimeCardTransaction{
            its_date: date,
            its_hours: hours,
            its_empid: empid,
        }
    }
}

impl Transaction for TimeCardTransaction {
    fn execute(&self) -> Result<(), Error> {
        GLOBAL_PAYROLL_DB.with(|connection| {
            let db = connection.borrow();
            let employee = db.get_employee(self.its_empid);
            let classification = employee.get_classification();
            let mut classification_cell = classification.borrow_mut();
            let any_classification = classification_cell.as_mut_any();
            let hc = any_classification.downcast_mut::<HourlyClassification>();
            match hc {
                Some(hc) => hc.add_time_card(TimeCard::new(self.its_date, self.its_hours)),
                None => bail!("Tried to add timecard to non-hourly employee"),
            }
            Ok(())
        });

        Ok(())
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
            let classification_cell = classification.borrow();
            assert!(classification_cell.as_any().downcast_ref::<SalariedClassification>().is_some());
            let sc = classification_cell.as_any().downcast_ref::<SalariedClassification>().unwrap();
            assert_eq!(sc.get_salary(), 1000.00);
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

        let tct = TimeCardTransaction::new(Local.ymd(2001,10,31), 8.0, emp_id);
        assert!(tct.execute().is_ok());

        GLOBAL_PAYROLL_DB.with(|connection| {
            let db = connection.borrow();
            let employee = db.get_employee(emp_id);

            let classification = employee.get_classification();
            let classification_cell = classification.borrow();
            assert!(classification_cell.as_any().downcast_ref::<HourlyClassification>().is_some());
            let hc = classification_cell.as_any().downcast_ref::<HourlyClassification>().unwrap();

            let d = Local.ymd(2001,10,31);
            let time_card = hc.get_time_card(&d);
            assert_eq!(8.0, time_card.unwrap().get_hours());
        });
    }
}
