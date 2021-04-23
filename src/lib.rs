pub trait TransactionRepository {
    fn add(&mut self, transaction_amount: isize);
}

pub struct AccountService<'a, T: TransactionRepository> {
    transaction_repository: &'a mut T,
}

impl<'a, T> AccountService<'a, T>
where
    T: TransactionRepository,
{
    pub fn new(repository: &'a mut T) -> Self {
        AccountService {
            transaction_repository: repository,
        }
    }

    pub fn deposit(&mut self, amount: usize) {
        self.transaction_repository.add(amount as isize);
    }

    pub fn withdraw(&mut self, amount: usize) {
        self.transaction_repository.add(-(amount as isize));
    }

    pub fn print_statement(&self) {
        todo!()
    }
}

#[cfg(test)]
mod account_service_tests {
    use super::AccountService;
    use crate::TestTransactionRepository;

    #[test]
    fn it_registers_a_deposit() {
        let mut repository = TestTransactionRepository::new();
        let expected: Vec<isize> = vec![1000];

        let mut account_service = AccountService::new(&mut repository);
        account_service.deposit(1000);

        assert_eq!(&expected, repository.transactions());
    }

    #[test]
    fn it_registers_a_withdraw() {
        let mut repository = TestTransactionRepository::new();
        let expected: Vec<isize> = vec![-1000];

        let mut account_service = AccountService::new(&mut repository);
        account_service.withdraw(1000);

        assert_eq!(&expected, repository.transactions());
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

    pub fn transactions(&self) -> &Vec<isize> {
        &self.transactions
    }
}

impl TransactionRepository for TestTransactionRepository {
    fn add(&mut self, transaction_amount: isize) {
        self.transactions.push(transaction_amount);
    }
}

pub struct OutputWriter {
    output: String,
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
