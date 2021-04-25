use bank_rust::{AccountService, transaction};
use std::io::{Write, Result as IoResult};

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

#[test]
fn it_prints_a_bank_statement() {
    let statement = format!(
        "Date | Amount | Balance\n{}\n{}\n{}\n",
        "14/01/2012 | -500 | 2500", "13/01/2012 | 2000 | 3000", "10/01/2012 | 1000 | 1000"
    );

    let mut output_writer = OutputWriter::new();
    let mut test_repository = transaction::InMemoryRepository::new();

    let mut account_service =
        AccountService::new(&mut test_repository, &mut output_writer);
    account_service.deposit(1000);
    account_service.deposit(2000);
    account_service.withdraw(500);
    account_service.print_statement().unwrap();

    assert_eq!(output_writer.output(), statement);
}
