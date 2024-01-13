use std::collections::HashMap;
use clap::ArgMatches;
use crate::counter::byte::ByteCounter;
use crate::counter::character::CharacterCounter;
use crate::counter::CounterPostProcessor;
use crate::counter::line::LineCounter;
use crate::counter::manager::CounterManager;
use crate::counter::max_line_length::MaxLineLengthCounter;
use crate::counter::word::WordCounter;
use crate::file_details::FileDetails;
use crate::counter::Counter;

pub struct MultiCounterManager {
    pub counters: HashMap<String, CounterManager>
}

impl MultiCounterManager {
    pub fn new(file_names: Vec<String>) -> Self {
        let mut instance = MultiCounterManager {
            counters: HashMap::with_capacity(file_names.len())
        };

        for file_name in file_names {
            instance.counters.insert(file_name, CounterManager::new());
        }

        instance
    }

    pub fn assign_flags(mut self, match_results: &ArgMatches) -> Self {
        let mut is_bytes_flag = match_results.get_flag(ByteCounter::ARG_ID);
        let is_chars_flag = match_results.get_flag(CharacterCounter::ARG_ID);
        let mut is_lines_flag = match_results.get_flag(LineCounter::ARG_ID);
        let is_max_line_length_flag = match_results.get_flag(MaxLineLengthCounter::ARG_ID);
        let mut is_words_flag = match_results.get_flag(WordCounter::ARG_ID);
        let is_default = !(is_bytes_flag || is_chars_flag || is_lines_flag || is_max_line_length_flag || is_words_flag || is_max_line_length_flag);
        if is_default {
            is_lines_flag = true;
            is_words_flag = true;
            is_bytes_flag = true;
        }

        for (_, counter) in &mut self.counters {
            counter.assign_flag(
                is_bytes_flag,
                is_chars_flag,
                is_words_flag,
                is_lines_flag,
                is_max_line_length_flag
            );
        }

        self
    }

    pub fn get_count_manager(&mut self, file_path: &str) -> Option<&mut CounterManager> {
        self.counters.get_mut(file_path)
    }

    pub fn respond_total(&mut self) -> Option<CounterManager> {
        if self.counters.len() <= 1 {
            return None;
        }

        let mut total_count_manager =
            CounterManager::new_with_flags(self.counters.iter().next().unwrap().1);

        for (_, counter_manager) in &mut self.counters {
            total_count_manager.add_count(counter_manager);
        }

        Some(total_count_manager)
    }

    pub fn process(mut self, file_details: Vec<FileDetails>) -> Self {
        for file_detail in file_details {
            let file_contents = file_detail.contents;
            let file_path = file_detail.file_path;
            let count_manager = self.get_count_manager(&file_path).unwrap();
            MultiCounterManager::process_per_file(file_contents, count_manager);
            println!("{} {}", count_manager.respond(), file_path);
        }
        self
    }

    fn process_per_file(file_contents: String, counter_manager: &mut CounterManager) {
        let mut iter = file_contents.chars().into_iter();
        while let Some(char) = iter.next() {
            counter_manager.count(char);
        }
        counter_manager.post_process();
    }

    pub fn post_process(mut self) -> Self {
        match self.respond_total() {
            Some(mut total_counter_manager) => {
                println!("{} {}", total_counter_manager.respond(), "total")
            },
            None => {}
        }

        self
    }
}