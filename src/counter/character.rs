use crate::counter::{Counter, CounterDetails};

#[derive(Clone)]
pub struct CharacterCounter {
    pub counter: usize,
    pub details: CounterDetails
}

impl Counter for CharacterCounter {
    fn count(&mut self, char: char) {
        self.add_count(1);
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