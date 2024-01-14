mod constants;
mod counter;
mod cmd;
mod multi_counter;
mod file_details;

use std::io::{BufRead, Read};
use std::ops::Deref;
use clap::ArgMatches;
use clap::parser::ValuesRef;
use crate::constants::FileArg;
use crate::counter::{Counter, CounterPostProcessor};
use crate::counter::byte::ByteCounter;
use crate::counter::character::CharacterCounter;
use crate::counter::line::LineCounter;
use crate::counter::manager::CounterManager;
use crate::counter::max_line_length::MaxLineLengthCounter;
use crate::counter::word::WordCounter;
use crate::multi_counter::MultiCounterManager;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    let match_results = cmd::config();
    let mut cmd_args_manager =
        CmdArgManager::new().check_flags(&match_results);
    let file_details =
        file_details::get_contents_from_files_or_stdin(&mut cmd_args_manager);
    let file_names = file_details::get_file_names(&file_details);

    MultiCounterManager::new(file_names)
        .assign_flags(&mut cmd_args_manager)
        .process(file_details)
        .post_process();
}

struct CmdArgManager<'a> {
    byte_flag: bool,
    character_flag: bool,
    word_flag: bool,
    line_flag: bool,
    max_line_length_flag: bool,
    input_from_file: bool,
    input_from_stdin: bool,
    file_paths_option: Option<ValuesRef<'a ,String>>
}

impl<'a> CmdArgManager<'a> {

    pub fn new() -> Self {
        CmdArgManager {
            byte_flag: false,
            character_flag: false,
            word_flag: false,
            line_flag: false,
            max_line_length_flag: false,
            input_from_file: false,
            input_from_stdin: false,
            file_paths_option: None
        }
    }
    pub fn check_flags(mut self, match_results: &'a ArgMatches) -> Self {
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

        self.byte_flag = is_bytes_flag;
        self.character_flag = is_chars_flag;
        self.word_flag = is_words_flag;
        self.line_flag = is_lines_flag;
        self.max_line_length_flag = is_max_line_length_flag;

        let file_path_option = match_results.get_many::<String>(FileArg::ID);


        if file_path_option.is_some() {
            self.input_from_file = true;
            self.file_paths_option = file_path_option;
        } else {
            self.input_from_stdin = true;
        }

        self
    }

    pub fn assign_flags(&mut self, counter_manager: &mut CounterManager) {
        counter_manager.byte.details.is_requested = self.byte_flag;
        counter_manager.character.details.is_requested = self.character_flag;
        counter_manager.word.details.is_requested = self.word_flag;
        counter_manager.line.details.is_requested = self.line_flag;
        counter_manager.max_line_length.details.is_requested = self.max_line_length_flag;
    }

    pub fn is_input_from_files(&self) -> bool {
        self.input_from_file
    }

    pub fn is_input_from_stdin(&self) -> bool {
        self.input_from_stdin
    }
}
