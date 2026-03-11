use lib_module::app::{users, auth};

fn main() {
    users::create_user("Samaddy");
    users::delete_user("John");
    auth::login("Samaddy", "my_pass");
}