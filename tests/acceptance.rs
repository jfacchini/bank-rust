use bank_rust::clock::Clock;
use bank_rust::{transaction, AccountService};
use chrono::{Date, TimeZone, Utc};
use std::cell::RefCell;
use std::io::{Result as IoResult, Write};

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

struct MockClock {
    dates: RefCell<Vec<Date<Utc>>>,
}

impl MockClock {
    fn new() -> Self {
        MockClock {
            dates: RefCell::new(vec![
                Utc.ymd(2012, 01, 14),
                Utc.ymd(2012, 01, 13),
                Utc.ymd(2012, 01, 10),
            ]),
        }
    }
}

impl Clock for MockClock {
    fn now(&self) -> Date<Utc> {
        self.dates.borrow_mut().pop().unwrap()
    }
}

#[test]
fn it_prints_a_bank_statement() {
    let statement = format!(
        "Date | Amount | Balance\n{}\n{}\n{}\n",
        "14/01/2012 | -500 | 2500", "13/01/2012 | 2000 | 3000", "10/01/2012 | 1000 | 1000"
    );

    let mut output_writer = OutputWriter::new();
    let test_repository = transaction::InMemoryRepository::new();

    let mut account_service =
        AccountService::new(test_repository, &mut output_writer, MockClock::new());
    account_service.deposit(1000);
    account_service.deposit(2000);
    account_service.withdraw(500);
    account_service.print_statement().unwrap();

    assert_eq!(output_writer.output(), statement);
}
