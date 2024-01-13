pub mod byte;
pub mod character;
pub mod word;
pub mod line;
pub mod max_line_length;
pub mod manager;


pub trait Counter {
    fn count(&mut self, char: char);
    fn count_if_requested(&mut self, char: char);
    fn add_count(&mut self, count: usize) {}
    fn get_count(&mut self) -> usize;

    // fn post_process(&mut self) {}
    // fn post_process_if_requested(&mut self) {}
}

pub trait CounterPostProcessor {
    fn post_process(&mut self);
    fn post_process_if_requested(&mut self);
}

#[derive(Clone)]
struct CounterDetails {
    pub is_requested: bool
}

pub enum CounterType {
    Byte,
    Character,
    Word,
    Line,
    MaxLineLength
}