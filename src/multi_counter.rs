use std::collections::HashMap;
use clap::ArgMatches;
use crate::CmdArgManager;
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

    pub fn assign_flags(mut self, cmd_arg_manager: &mut CmdArgManager) -> Self {

        for (_, counter) in &mut self.counters {
            cmd_arg_manager.assign_flags(counter);
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