use std::cmp::max;
use crate::counter::{Counter, CounterDetails, CounterPostProcessor};

#[derive(Clone)]
pub struct WordCounter {
    pub counter: usize,
    pub word_length: usize,
    pub details: CounterDetails
}

impl Counter for WordCounter {
    fn count(&mut self, char: char) {
        if char.is_whitespace() {
            if self.word_length > 0 {
                self.add_count(1)
            }
            self.word_length = 0;
        } else {
            self.word_length += 1;
        }
    }
    fn count_if_requested(&mut self, char: char) {
        if self.details.is_requested {
            self.count(char);
        }
    }

    fn add_count(&mut self, counter: usize) {
        self.counter += counter;
    }

    fn get_count(&mut self) -> usize {
        self.counter
    }
}

impl CounterPostProcessor for WordCounter {
    fn post_process(&mut self) {
        if self.word_length > 0 {
            self.counter += 1;
        }
        self.word_length = 0;
    }
    fn post_process_if_requested(&mut self) {
        if self.details.is_requested {
            self.post_process()
        }
    }
}