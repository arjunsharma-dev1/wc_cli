use crate::counter::{CounterPostProcessor, CounterType};
use crate::counter::byte::ByteCounter;
use crate::counter::character::CharacterCounter;
use crate::counter::{Counter, CounterDetails};
use crate::counter::word::WordCounter;
use crate::counter::line::LineCounter;
use crate::counter::max_line_length::MaxLineLengthCounter;

#[derive(Clone)]
pub struct CounterManager {
    pub byte: ByteCounter,
    pub character: CharacterCounter,
    pub line: LineCounter,
    pub max_line_length: MaxLineLengthCounter,
    pub word: WordCounter
}

impl CounterManager {
    pub fn new() -> Self {
        CounterManager {
            byte: ByteCounter{
                counter: 0,
                details: CounterDetails {
                    is_requested: false
                }
            },
            character: CharacterCounter{
                counter: 0,
                details: CounterDetails {
                    is_requested: false
                }
            },
            line: LineCounter {
                counter: 0,
                details: CounterDetails {
                    is_requested: false
                },
                line_char_counter: 0
            },
            word: WordCounter {
                word_length: 0,
                counter: 0,
                details: CounterDetails {
                    is_requested: false
                }
            },
            max_line_length: MaxLineLengthCounter {
                line_length: 0,
                max_line_length: 0,
                details: CounterDetails {
                    is_requested: false
                }
            }
        }
    }

    pub fn new_with_flags(counter_manager: &CounterManager) -> Self {
        CounterManager {
            byte: ByteCounter{
                counter: 0,
                details: CounterDetails {
                    is_requested: counter_manager.byte.details.is_requested
                }
            },
            character: CharacterCounter{
                counter: 0,
                details: CounterDetails {
                    is_requested: counter_manager.character.details.is_requested
                }
            },
            line: LineCounter {
                counter: 0,
                details: CounterDetails {
                    is_requested: counter_manager.line.details.is_requested
                },
                line_char_counter: 0
            },
            word: WordCounter {
                word_length: 0,
                counter: 0,
                details: CounterDetails {
                    is_requested: counter_manager.word.details.is_requested
                }
            },
            max_line_length: MaxLineLengthCounter {
                line_length: 0,
                max_line_length: 0,
                details: CounterDetails {
                    is_requested: counter_manager.max_line_length.details.is_requested
                }
            }
        }
    }

    pub fn respond(&mut self) -> String {
        let mut response = String::new();
        if self.line.details.is_requested {
            self.append_result(&mut response, CounterType::Line)
        }
        if self.word.details.is_requested {
            self.append_result(&mut response, CounterType::Word)
        }
        if self.character.details.is_requested {
            self.append_result(&mut response, CounterType::Character)
        }
        if self.byte.details.is_requested {
            self.append_result(&mut response, CounterType::Byte)
        }
        if self.max_line_length.details.is_requested {
            self.append_result(&mut response, CounterType::MaxLineLength)
        }
        response
    }
    fn get_counter(&mut self, counter_type: CounterType) -> Box<&mut dyn Counter> {
        match counter_type {
            CounterType::Byte => Box::new(&mut self.byte),
            CounterType::Character => Box::new(&mut self.character),
            CounterType::Word => Box::new(&mut self.word),
            CounterType::Line => Box::new(&mut self.line),
            CounterType::MaxLineLength => Box::new(&mut self.max_line_length)
        }
    }
    fn append_result(&mut self, response: &mut String, counter_type: CounterType) {
        if !response.is_empty() {
            response.push(' ');
        }
        response.push_str(self.get_counter(counter_type).get_count().to_string().as_ref());
    }
    pub fn add_count(&mut self, counter_manager: &mut CounterManager) {
        self.byte.add_count(counter_manager.byte.get_count());
        self.character.add_count(counter_manager.character.get_count());
        self.word.add_count(counter_manager.word.get_count());
        self.line.add_count(counter_manager.line.get_count());
        self.max_line_length.add_count(counter_manager.max_line_length.get_count());
    }
}

impl CounterManager {
    pub fn assign_flag(
        &mut self,
        byte_requested: bool,
        character_requested: bool,
        word_requested: bool,
        line_requested: bool,
        max_line_length_requested: bool
    ) {
        self.byte.details.is_requested = byte_requested;
        self.character.details.is_requested = character_requested;
        self.word.details.is_requested = word_requested;
        self.line.details.is_requested = line_requested;
        self.max_line_length.details.is_requested = max_line_length_requested;
    }
}

impl Counter for CounterManager {
    fn count(&mut self, char: char) {
        self.byte.count_if_requested(char);
        self.character.count_if_requested(char);
        self.word.count_if_requested(char);
        self.line.count_if_requested(char);
        self.max_line_length.count_if_requested(char);
    }

    fn count_if_requested(&mut self, char: char) { todo!() }

    fn get_count(&mut self) -> usize { todo!() }
}

impl CounterPostProcessor for CounterManager {
    fn post_process(&mut self) {
        let word_counter = &mut self.word as &mut dyn CounterPostProcessor;
        let max_line_length_counter = &mut self.max_line_length as &mut dyn CounterPostProcessor;

        word_counter.post_process_if_requested();
        max_line_length_counter.post_process_if_requested();
    }

    fn post_process_if_requested(&mut self) {
        self.post_process()
    }
}