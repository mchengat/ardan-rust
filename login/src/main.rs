use auth::{greet_user, is_login_allowed, login, LoginAction, LoginRole};

fn read_line() -> String {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
fn main() {
    let mut count = 1;
    loop {
        println!("Enter the username: ");
        let username = read_line();

        println!("Enter the password: ");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(LoginRole::Admin)) => {
                println!("Welcome admin");
                break;
            }
            Some(LoginAction::Granted(LoginRole::User)) => {
                println!("Welcome Bob");
                break;
            }
            Some(LoginAction::Denied) | None => {
                if count == 3 {
                    println!("Go Away");
                    break;
                }
                count += 1;
            }
        }
    }
}
