use crate::counter::byte::ByteCounter;
use crate::counter::character::CharacterCounter;
use crate::counter::line::LineCounter;
use crate::counter::max_line_length::MaxLineLengthCounter;
use crate::counter::word::WordCounter;

pub const NEW_LINE: char = 0xA as char;
impl ByteCounter {
    pub const ARG_ID: &'static str = "bytes";
    pub const ARG_SHORT: char = 'c';
    pub const ARG_LONG: &'static str = "bytes";
}


impl CharacterCounter {
    pub const ARG_ID: &'static str = "chars";
    pub const ARG_SHORT: char = 'm';
    pub const ARG_LONG: &'static str = "chars";
}

impl WordCounter {
    pub const ARG_ID: &'static str = "words";
    pub const ARG_SHORT: char = 'w';
    pub const ARG_LONG: &'static str = "words";
}

impl LineCounter {
    pub const ARG_ID: &'static str = "lines";
    pub const ARG_SHORT: char = 'l';
    pub const ARG_LONG: &'static str = "lines";
}

impl MaxLineLengthCounter {
    pub const ARG_ID: &'static str = "max-line-length";
    pub const ARG_SHORT: char = 'L';
    pub const ARG_LONG: &'static str = "max-line-length";
}

pub struct FileArg;
impl FileArg {
    pub const ID: &'static str = "file";
    pub const SHORT: char = 'f';
    pub const LONG: &'static str = "files";
    pub const VALUE_NAME: &'static str = "FILE";

}