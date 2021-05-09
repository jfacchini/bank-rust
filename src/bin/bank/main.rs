use bank_rust::clock::SystemClock;
use bank_rust::{transaction::InMemoryRepository, AccountService};

fn main() {
    let mut account_service =
        AccountService::new(InMemoryRepository::new(), SystemClock);

    account_service.deposit(1000);
    account_service.deposit(400);
    account_service
        .print_statement(std::io::stdout())
        .expect("Unable to print statement");
}
