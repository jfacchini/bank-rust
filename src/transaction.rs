use chrono::{Utc, Date};

#[derive(PartialEq, Debug)]
pub struct Transaction {
    pub amount: isize,
    pub date: Date<Utc>
}

pub trait TransactionRepository {
    fn add(&mut self, transaction: Transaction);
    fn all(&self) -> &Vec<Transaction>;
}

pub struct InMemoryRepository {
    transactions: Vec<Transaction>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        InMemoryRepository {
            transactions: Vec::new(),
        }
    }
}

impl TransactionRepository for InMemoryRepository {
    fn add(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    fn all(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}
