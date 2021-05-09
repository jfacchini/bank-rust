pub mod clock;
pub mod transaction;

use crate::clock::Clock;
use crate::transaction::Transaction;
use std::io::{Result as IoResult, Write};
use transaction::TransactionRepository;

pub struct AccountService<T: TransactionRepository, C: Clock> {
    transaction_repository: T,
    clock: C,
}

const STATEMENT_HEADER: &str = "Date | Amount | Balance";

impl<T, C> AccountService<T, C>
where
    T: TransactionRepository,
    C: Clock,
{
    pub fn new(repository: T, clock: C) -> Self {
        AccountService {
            transaction_repository: repository,
            clock,
        }
    }

    pub fn deposit(&mut self, amount: isize) {
        if amount < 1 {
            panic!("A deposit need a positive value, got {}", amount);
        }

        self.transaction_repository.add(Transaction {
            amount,
            date: self.clock.now(),
        });
    }

    pub fn withdraw(&mut self, amount: isize) {
        if amount < 1 {
            panic!("A withdraw need a positive value, got {}", amount);
        }

        self.transaction_repository.add(Transaction {
            amount: -amount,
            date: self.clock.now(),
        });
    }

    pub fn print_statement(&self, mut output_writer: impl Write) -> IoResult<()> {
        writeln!(output_writer, "{}", STATEMENT_HEADER)?;

        let mut total = 0;
        let statement_lines = self
            .all_transactions()
            .iter()
            .map(|Transaction { amount, date }| {
                total += amount;
                format!(
                    "{} | {} | {}",
                    date.format("%d/%m/%Y").to_string(),
                    amount,
                    total
                )
            })
            .collect::<Vec<String>>();

        for line in statement_lines.iter().rev() {
            writeln!(output_writer, "{}", line)?;
        }

        IoResult::Ok(())
    }

    fn all_transactions(&self) -> &Vec<Transaction> {
        self.transaction_repository.all()
    }
}

#[cfg(test)]
mod account_service_tests {
    use super::*;
    use chrono::{Date, TimeZone, Utc};

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
            MockClock,
        );
        account_service.deposit(1000);

        let repository = &account_service.transaction_repository;
        assert_eq!(&expected, repository.all());
    }

    #[test]
    #[should_panic]
    fn it_does_not_allow_negative_deposit() {
        let mut account_service = AccountService::new(
            transaction::InMemoryRepository::new(),
            MockClock,
        );
        account_service.deposit(-1000);
    }

    #[test]
    fn it_registers_a_withdraw() {
        let expected: Vec<Transaction> = vec![Transaction {
            amount: -1000,
            date: Utc.ymd(2015, 12, 01),
        }];

        let mut account_service = AccountService::new(
            transaction::InMemoryRepository::new(),
            MockClock,
        );
        account_service.withdraw(1000);

        let repository = &account_service.transaction_repository;
        assert_eq!(&expected, repository.all());
    }

    #[test]
    #[should_panic]
    fn it_does_not_allow_negative_withdraw() {
        let mut account_service = AccountService::new(
            transaction::InMemoryRepository::new(),
            MockClock,
        );
        account_service.withdraw(-1000);
    }
}
