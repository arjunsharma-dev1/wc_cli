mod constants;
mod counter;
mod cmd;
mod multi_counter;
mod file_details;

use std::io::{BufRead, Read};
use std::ops::Deref;
use crate::counter::{Counter, CounterPostProcessor};
use crate::multi_counter::MultiCounterManager;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let match_results = cmd::config();
    let file_details = file_details::get_contents_from_files_or_stdin(&match_results);
    // TODO: what if, if stdin?
    let file_names = file_details::get_file_names(&file_details);

    MultiCounterManager::new(file_names)
        .assign_flags(&match_results)
        .process(file_details)
        .post_process();
}



