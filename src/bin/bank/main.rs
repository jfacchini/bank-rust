use bank_rust::{transaction::InMemoryRepository, AccountService};

fn main() {
    let mut repository = InMemoryRepository::new();
    let mut writer = std::io::stdout();
    let mut account_service = AccountService::new(&mut repository, &mut writer);

    account_service.deposit(1000);
    account_service.deposit(400);
    account_service
        .print_statement()
        .expect("Unable to print statement");
}
