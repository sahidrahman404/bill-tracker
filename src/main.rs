use std::io;

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

enum MainMenu {
    AddBill,
    ViewBill,
}

impl MainMenu {
    fn select_menu(input: String) -> Option<MainMenu> {
        match input.as_str() {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            _ => None,
        }
    }
    fn show_menu() {
        println!("");
        println!("== Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("");
        println!("Enter Selection: ");
    }
}

fn main() {
    // create bill structure
    loop {
        // Display the menu
        MainMenu::show_menu();
        let input = get_input().expect("no data entered");
        match MainMenu::select_menu(input) {
            Some(MainMenu::AddBill) => (),
            Some(MainMenu::ViewBill) => (),
            None => return,
        }
        // Make a choice, based on user input
    }
}
