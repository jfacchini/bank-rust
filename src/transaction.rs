pub trait TransactionRepository {
    fn add(&mut self, transaction_amount: isize);
    fn all(&self) -> &Vec<isize>;
}

pub struct InMemoryRepository {
    transactions: Vec<isize>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            transactions: Vec::new(),
        }
    }
}

impl TransactionRepository for InMemoryRepository {
    fn add(&mut self, transaction_amount: isize) {
        self.transactions.push(transaction_amount);
    }

    fn all(&self) -> &Vec<isize> {
        &self.transactions
    }
}
