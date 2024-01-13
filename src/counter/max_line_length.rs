use std::cmp::max;
use crate::constants;
use crate::counter::{Counter, CounterDetails, CounterPostProcessor};

#[derive(Clone)]
pub struct MaxLineLengthCounter {
    pub max_line_length: usize,
    pub line_length: usize,
    pub details: CounterDetails
}

impl Counter for MaxLineLengthCounter {
    fn count(&mut self, char: char) {
        if char == constants::NEW_LINE {
            self.add_count(self.line_length);
            self.line_length = 0;
        } else {
            self.line_length += 1;
        }
    }
    fn count_if_requested(&mut self, char: char) {
        if self.details.is_requested {
            self.count(char);
        }
    }

    fn add_count(&mut self, line_length: usize) {
        self.max_line_length = max(line_length, self.max_line_length);
    }

    fn get_count(&mut self) -> usize {
        self.max_line_length
    }
}

impl CounterPostProcessor for MaxLineLengthCounter {
    fn post_process(&mut self) {
        self.max_line_length = max(self.line_length, self.max_line_length)
    }
    fn post_process_if_requested(&mut self) {
        if self.details.is_requested {
            self.post_process()
        }
    }
}