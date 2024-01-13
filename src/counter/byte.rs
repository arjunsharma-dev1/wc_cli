use crate::counter::{Counter, CounterDetails};

#[derive(Clone)]
pub struct ByteCounter {
    pub counter: usize,
    pub details: CounterDetails
}

impl Counter for ByteCounter {
    fn count(&mut self, char: char) {
        self.add_count(char.len_utf8());
    }

    fn count_if_requested(&mut self, char: char) {
        if self.details.is_requested {
            self.count(char);
        }
    }

    fn add_count(&mut self, count: usize) {
        self.counter += count;
    }

    fn get_count(&mut self) -> usize {
        self.counter
    }
}