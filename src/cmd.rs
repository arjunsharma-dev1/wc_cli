use clap::{Arg, arg, ArgAction, ArgMatches, command};
use clap::builder::ValueParser;
use crate::counter::byte::ByteCounter;
use crate::counter::character::CharacterCounter;
use crate::counter::line::LineCounter;
use crate::counter::max_line_length::MaxLineLengthCounter;
use crate::counter::word::WordCounter;
use crate::constants::{FileArg};

pub fn config() -> ArgMatches {
    command!()
        .arg(byte_arg())
        .arg(character_arg())
        .arg(line_arg())
        .arg(max_line_length_arg())
        .arg(word_arg())
        .arg(file_arg())
        .get_matches()
}

fn byte_arg() -> Arg {
    Arg::new(ByteCounter::ARG_ID)
        .short(ByteCounter::ARG_SHORT)
        .long(ByteCounter::ARG_LONG)
        .value_parser(ValueParser::bool())
        .action(ArgAction::SetTrue)
}

fn character_arg() -> Arg {
    Arg::new(CharacterCounter::ARG_ID)
        .short(CharacterCounter::ARG_SHORT)
        .long(CharacterCounter::ARG_LONG)
        .value_parser(ValueParser::bool())
        .action(ArgAction::SetTrue)
}

fn word_arg() -> Arg {
    Arg::new(WordCounter::ARG_ID)
        .short(WordCounter::ARG_SHORT)
        .long(WordCounter::ARG_LONG)
        .value_parser(ValueParser::bool())
        .action(ArgAction::SetTrue)
}

fn max_line_length_arg() -> Arg {
    Arg::new(MaxLineLengthCounter::ARG_ID)
        .short(MaxLineLengthCounter::ARG_SHORT)
        .long(MaxLineLengthCounter::ARG_LONG)
        .value_parser(ValueParser::bool())
        .action(ArgAction::SetTrue)
}

fn line_arg() -> Arg {
    Arg::new(LineCounter::ARG_ID)
        .short(LineCounter::ARG_SHORT)
        .long(LineCounter::ARG_LONG)
        .value_parser(ValueParser::bool())
        .action(ArgAction::SetTrue)
}

fn file_arg() -> Arg {
    // arg!(-f --file <FILE>)
    Arg::new(FileArg::ID)
        .short(FileArg::SHORT)
        .long(FileArg::LONG)
        .value_name(FileArg::VALUE_NAME)
        .num_args(1..)
}