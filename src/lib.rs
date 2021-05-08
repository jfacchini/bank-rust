pub mod clock;
pub mod transaction;

use crate::clock::Clock;
use crate::transaction::Transaction;
use std::io::{Result as IoResult, Write};
use transaction::TransactionRepository;

pub struct AccountService<T: TransactionRepository, W: Write, C: Clock> {
    transaction_repository: T,
    output_writer: W,
    clock: C,
}

const STATEMENT_HEADER: &str = "Date | Amount | Balance";

impl<T, W, C> AccountService<T, W, C>
where
    T: TransactionRepository,
    W: Write,
    C: Clock,
{
    pub fn new(repository: T, writer: W, clock: C) -> Self {
        AccountService {
            transaction_repository: repository,
            output_writer: writer,
            clock,
        }
    }

    pub fn deposit(&mut self, amount: usize) {
        self.transaction_repository.add(Transaction {
            amount: amount as isize,
            date: self.clock.now(),
        });
    }

    pub fn withdraw(&mut self, amount: usize) {
        self.transaction_repository.add(Transaction {
            amount: -(amount as isize),
            date: self.clock.now(),
        });
    }

    pub fn print_statement(&mut self) -> IoResult<()> {
        writeln!(self.output_writer, "{}", STATEMENT_HEADER)?;

        let mut statement_lines = Vec::new();
        let mut total = 0;
        for transaction in self.transaction_repository.all() {
            total += transaction.amount;
            statement_lines.push(format!(
                "{} | {} | {}",
                transaction.date.format("%d/%m/%Y").to_string(),
                transaction.amount,
                total
            ))
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
    use chrono::{Date, TimeZone, Utc};

    struct MockOutputWriter();

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

    struct MockClock;

    impl Clock for MockClock {
        fn now(&self) -> Date<Utc> {
            Utc.ymd(2015, 12, 01)
        }
    }

    #[test]
    fn it_registers_a_deposit() {
        let expected: Vec<Transaction> = vec![Transaction {
            amount: 1000,
            date: Utc.ymd(2015, 12, 01),
        }];

        let mut account_service = AccountService::new(
            transaction::InMemoryRepository::new(),
            MockOutputWriter::new(),
            MockClock,
        );
        account_service.deposit(1000);

        let repository = &account_service.transaction_repository;
        assert_eq!(&expected, repository.all());
    }

    #[test]
    fn it_registers_a_withdraw() {
        let expected: Vec<Transaction> = vec![Transaction {
            amount: -1000,
            date: Utc.ymd(2015, 12, 01),
        }];

        let mut account_service = AccountService::new(
            transaction::InMemoryRepository::new(),
            MockOutputWriter::new(),
            MockClock,
        );
        account_service.withdraw(1000);

        let repository = &account_service.transaction_repository;
        assert_eq!(&expected, repository.all());
    }
}
