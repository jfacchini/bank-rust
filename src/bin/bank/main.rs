use bank_rust::{AccountService, TestTransactionRepository};

fn main() {
    let mut repository = TestTransactionRepository::new();
    let mut writer = std::io::stdout();
    let mut account_service = AccountService::new(&mut repository, &mut writer);

    account_service.deposit(1000);
    account_service.deposit(400);
    account_service.print_statement();
}
