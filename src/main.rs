use std::{collections::HashMap, io};

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_lowercase();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn get_bill_amount() -> Option<f64> {
    println!("Amount");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("please enter a number"),
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
}

impl MainMenu {
    fn select_menu(input: String) -> Option<MainMenu> {
        match input.as_str() {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            _ => None,
        }
    }
    fn show_menu() {
        println!("");
        println!("== Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bills");
        println!("");
        println!("Enter Selection: ");
    }
}

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};
    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("bill added");
    }
    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }

    pub fn remove_bills(bill: &mut Bills) {
        for bill in bill.get_all() {
            println!("{:?}", bill)
        }
        println!("Enter bill name to remove");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        if bill.remove(&name) {
            println!("bill removed");
        } else {
            println!("bill not found");
        };
    }
}

fn main() {
    // create bill structure
    let mut bills = Bills::new();

    loop {
        // Display the menu
        MainMenu::show_menu();
        let input = get_input().expect("no data entered");
        match MainMenu::select_menu(input) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bills(&mut bills),
            None => return,
        }
        // Make a choice, based on user input
    }
}
