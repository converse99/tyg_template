fn main() {
    match tyg_template::run() {
        Ok(_) => println!("The process completed normally"),
        Err(e) => eprintln!("tyg_template: {}", e),
    }
}
