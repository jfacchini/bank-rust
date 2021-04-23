use bank_rust;
use bank_rust::TestTransactionRepository;

#[test]
fn it_prints_a_bank_statement() {
    let statement = format!(
        "Date | Amount | Balance\n{}\n{}\n{}\n",
        "14/01/2012 | -500 | 2500", "13/01/2012 | 2000 | 3000", "10/01/2012 | 1000 | 1000"
    );

    let mut output_writer = bank_rust::OutputWriter::new();
    let mut test_repository = TestTransactionRepository::new();

    let mut account_service =
        bank_rust::AccountService::new(&mut test_repository, &mut output_writer);
    account_service.deposit(1000);
    account_service.deposit(2000);
    account_service.withdraw(500);
    account_service.print_statement();

    assert_eq!(output_writer.output(), statement);
}
