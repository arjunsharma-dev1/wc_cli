use crate::counter::{Counter, CounterDetails, CounterPostProcessor};
use crate::constants;

#[derive(Clone)]
pub struct LineCounter {
    pub counter: usize,
    pub line_char_counter: usize,
    pub details: CounterDetails
}

impl Counter for LineCounter {
    fn count(&mut self, char: char) {
        if char == constants::NEW_LINE {
            self.add_count(1)
        }
        self.line_char_counter += 1;
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