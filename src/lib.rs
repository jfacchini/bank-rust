use std::io::{Result as IoResult, Write};

pub trait TransactionRepository {
    fn add(&mut self, transaction_amount: isize);
    fn all(&self) -> &Vec<isize>;
}

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

    pub fn print_statement(&mut self) {
        self.output_writer
            .write_all(format!("{}\n", STATEMENT_HEADER).as_bytes()).unwrap();

        let mut statement_lines = Vec::new();
        let mut total = 0;
        for transaction in self.transaction_repository.all() {
            total += transaction;
            statement_lines.push(format!("01/01/2021 | {} | {}\n", transaction, total))
        }

        for line in statement_lines.into_iter().rev() {
            self.output_writer
                .write_all(line.as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod account_service_tests {
    use super::*;
    use std::io::{Result as IoResult, Write};

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

        let mut repository = TestTransactionRepository::new();
        let mut writer = MockOutputWriter::new();

        let mut account_service = AccountService::new(&mut repository, &mut writer);
        account_service.deposit(1000);

        assert_eq!(&expected, repository.all());
    }

    #[test]
    fn it_registers_a_withdraw() {
        let expected: Vec<isize> = vec![-1000];

        let mut repository = TestTransactionRepository::new();
        let mut writer = MockOutputWriter::new();

        let mut account_service = AccountService::new(&mut repository, &mut writer);
        account_service.withdraw(1000);

        assert_eq!(&expected, repository.all());
    }
}

pub struct TestTransactionRepository {
    transactions: Vec<isize>,
}

impl TestTransactionRepository {
    pub fn new() -> Self {
        TestTransactionRepository {
            transactions: Vec::new(),
        }
    }
}

impl TransactionRepository for TestTransactionRepository {
    fn add(&mut self, transaction_amount: isize) {
        self.transactions.push(transaction_amount);
    }

    fn all(&self) -> &Vec<isize> {
        &self.transactions
    }
}

pub struct OutputWriter {
    output: String,
}

impl Write for OutputWriter {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        self.output = format!("{}{}", self.output, String::from_utf8_lossy(buf));

        IoResult::Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        IoResult::Ok(())
    }
}

impl OutputWriter {
    pub fn new() -> Self {
        OutputWriter {
            output: String::new(),
        }
    }

    pub fn output(&self) -> &str {
        &self.output
    }
}
