pub mod transaction;

use std::io::{Result as IoResult, Write};
use transaction::TransactionRepository;

pub struct AccountService<'a, 'b, T: TransactionRepository, W: Write> {
    transaction_repository: &'a mut T,
    output_writer: &'b mut W,
}

const STATEMENT_HEADER: &str = "Date | Amount | Balance";

impl<'a, 'b, T, W> AccountService<'a, 'b, T, W>
where
    T: TransactionRepository,
    W: Write,
{
    pub fn new(repository: &'a mut T, writer: &'b mut W) -> Self {
        AccountService {
            transaction_repository: repository,
            output_writer: writer,
        }
    }

    pub fn deposit(&mut self, amount: usize) {
        self.transaction_repository.add(amount as isize);
    }

    pub fn withdraw(&mut self, amount: usize) {
        self.transaction_repository.add(-(amount as isize));
    }

    pub fn print_statement(&mut self) -> IoResult<()> {
        writeln!(self.output_writer, "{}", STATEMENT_HEADER)?;

        let mut statement_lines = Vec::new();
        let mut total = 0;
        for transaction in self.transaction_repository.all() {
            total += transaction;
            statement_lines.push(format!("01/01/2021 | {} | {}", transaction, total))
        }

        for line in statement_lines.into_iter().rev() {
            writeln!(self.output_writer, "{}", line)?;
        }

        IoResult::Ok(())
    }
}

#[cfg(test)]
mod account_service_tests {
    use std::io::{Result as IoResult, Write};

    use super::*;

    struct MockOutputWriter {}
    impl MockOutputWriter {
        fn new() -> Self {
            MockOutputWriter {}
        }
    }
    impl Write for MockOutputWriter {
        fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
            IoResult::Ok(buf.len())
        }

        fn flush(&mut self) -> IoResult<()> {
            IoResult::Ok(())
        }
    }

    #[test]
    fn it_registers_a_deposit() {
        let expected: Vec<isize> = vec![1000];

        let mut repository = transaction::InMemoryRepository::new();
        let mut writer = MockOutputWriter::new();

        let mut account_service = AccountService::new(&mut repository, &mut writer);
        account_service.deposit(1000);

        assert_eq!(&expected, repository.all());
    }

    #[test]
    fn it_registers_a_withdraw() {
        let expected: Vec<isize> = vec![-1000];

        let mut repository = transaction::InMemoryRepository::new();
        let mut writer = MockOutputWriter::new();

        let mut account_service = AccountService::new(&mut repository, &mut writer);
        account_service.withdraw(1000);

        assert_eq!(&expected, repository.all());
    }
}
